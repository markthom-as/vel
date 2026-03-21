use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};
use time::OffsetDateTime;
use vel_config::{load_model_profiles, load_routing, ModelProfile};
use vel_llm::{
    LlamaCppConfig, LlamaCppProvider, LlmRequest, Message, OpenAiOauthConfig, OpenAiOauthProvider,
    ProviderRegistry, ResponseFormat, Router,
};
use vel_sim::{replay_day_scenario, DayScenarioFixture, ReplayReport, ScenarioKind};

pub const FIXTURE_SCHEMA_VERSION: &str = "veld_eval_fixture/v1";
pub const REPORT_SCHEMA_VERSION: &str = "veld_eval_report/v1";
const DEFAULT_MODELS_DIR: &str = "configs/models";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalFixtureSet {
    pub schema_version: String,
    pub scenarios: Vec<EvalScenarioFixture>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalScenarioFixture {
    pub name: String,
    pub kind: ScenarioKind,
    pub simulation: DayScenarioFixture,
    pub expectations: DeterministicExpectations,
    #[serde(default)]
    pub judge: JudgeConfig,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeterministicExpectations {
    pub expected_date: Option<String>,
    #[serde(default)]
    pub required_events: Vec<String>,
    pub min_ref_count: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum JudgeMode {
    #[default]
    Disabled,
    Router,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JudgeConfig {
    #[serde(default)]
    pub mode: JudgeMode,
    pub model_profile: Option<String>,
    #[serde(default)]
    pub rubric: Vec<String>,
    pub pass_score: Option<f32>,
    #[serde(default)]
    pub fail_on_regression: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalReport {
    pub schema_version: String,
    pub generated_at: String,
    pub summary: EvalSummary,
    pub scenarios: Vec<ScenarioReport>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalSummary {
    pub scenario_count: usize,
    pub passed_count: usize,
    pub deterministic_failure_count: usize,
    pub judge_failure_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioReport {
    pub name: String,
    pub status: ScenarioStatus,
    pub deterministic: DeterministicReport,
    pub judge: JudgeReport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScenarioStatus {
    Passed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeterministicReport {
    pub passed: bool,
    #[serde(default)]
    pub failures: Vec<String>,
    pub replay: Option<ReplayReport>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgeReport {
    pub mode: JudgeMode,
    pub attempted: bool,
    pub passed: bool,
    pub enforce_failure_policy: bool,
    pub skipped_reason: Option<String>,
    pub score: Option<f32>,
    pub threshold: Option<f32>,
    pub summary: Option<String>,
    #[serde(default)]
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct EvalRunOptions {
    pub fail_on_judge_regression: bool,
}

impl Default for EvalRunOptions {
    fn default() -> Self {
        Self {
            fail_on_judge_regression: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalExitCode {
    Success = 0,
    DeterministicFailure = 1,
    JudgeRegression = 2,
}

#[derive(Debug, Clone)]
pub struct JudgeDecision {
    pub score: f32,
    pub summary: String,
    pub reasons: Vec<String>,
}

#[async_trait::async_trait]
pub trait JudgeExecutor: Send + Sync {
    async fn evaluate(
        &self,
        scenario: &EvalScenarioFixture,
        replay: &ReplayReport,
        config: &JudgeConfig,
    ) -> Result<JudgeDecision>;
}

pub struct RouterJudge {
    router: Router,
    default_profile: Option<String>,
}

impl RouterJudge {
    pub fn new(router: Router, default_profile: Option<String>) -> Self {
        Self {
            router,
            default_profile,
        }
    }
}

#[async_trait::async_trait]
impl JudgeExecutor for RouterJudge {
    async fn evaluate(
        &self,
        scenario: &EvalScenarioFixture,
        replay: &ReplayReport,
        config: &JudgeConfig,
    ) -> Result<JudgeDecision> {
        let model_profile = config
            .model_profile
            .as_deref()
            .or(self.default_profile.as_deref())
            .ok_or_else(|| anyhow!("judge mode requested but no judge profile is configured"))?;
        let rubric = if config.rubric.is_empty() {
            vec![
                "Check whether the generated context is specific, grounded in inputs, and operator-usable."
                    .to_string(),
                "Fail if the output hallucinates information not present in the replayed scenario."
                    .to_string(),
            ]
        } else {
            config.rubric.clone()
        };
        let system = "You are Vel's evaluation judge. Score output quality from 0.0 to 1.0. Return strict JSON with keys: score, summary, reasons."
            .to_string();
        let user = serde_json::json!({
            "scenario_name": scenario.name,
            "scenario_kind": scenario.kind,
            "rubric": rubric,
            "context_json": replay.context_json,
            "boundary_events": replay.boundary_events,
        })
        .to_string();
        let request = LlmRequest {
            system,
            messages: vec![Message {
                role: "user".to_string(),
                content: user,
            }],
            tools: vec![],
            response_format: ResponseFormat::JsonObject,
            temperature: 0.0,
            max_output_tokens: 512,
            model_profile: model_profile.to_string(),
            metadata: serde_json::json!({
                "purpose": "eval_judge",
                "scenario": scenario.name,
            }),
        };
        let response = self.router.generate(&request).await?;
        let raw = response
            .text
            .clone()
            .or_else(|| {
                if response.raw.is_null() {
                    None
                } else {
                    Some(response.raw.to_string())
                }
            })
            .ok_or_else(|| anyhow!("judge response was empty"))?;
        let payload: JudgePayload =
            serde_json::from_str(&raw).context("parse judge JSON response")?;
        Ok(JudgeDecision {
            score: payload.score,
            summary: payload.summary,
            reasons: payload.reasons,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct JudgePayload {
    score: f32,
    summary: String,
    #[serde(default)]
    reasons: Vec<String>,
}

pub fn load_fixture_sets(path: &Path) -> Result<Vec<(PathBuf, EvalFixtureSet)>> {
    let mut files = Vec::new();
    if path.is_dir() {
        for entry in fs::read_dir(path).with_context(|| format!("read dir {}", path.display()))? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.extension().and_then(|v| v.to_str()) == Some("json") {
                files.push(entry_path);
            }
        }
        files.sort();
    } else {
        files.push(path.to_path_buf());
    }

    let mut sets = Vec::new();
    for fixture_path in files {
        let raw = fs::read_to_string(&fixture_path)
            .with_context(|| format!("read fixture {}", fixture_path.display()))?;
        let set: EvalFixtureSet = serde_json::from_str(&raw)
            .with_context(|| format!("parse fixture {}", fixture_path.display()))?;
        if set.schema_version != FIXTURE_SCHEMA_VERSION {
            bail!(
                "fixture {} uses unsupported schema_version {}",
                fixture_path.display(),
                set.schema_version
            );
        }
        sets.push((fixture_path, set));
    }
    Ok(sets)
}

pub async fn run_fixture_sets(
    fixture_sets: &[EvalFixtureSet],
    judge: Option<&dyn JudgeExecutor>,
    options: &EvalRunOptions,
) -> Result<EvalReport> {
    let mut scenarios = Vec::new();
    for fixture_set in fixture_sets {
        for scenario in &fixture_set.scenarios {
            scenarios.push(run_scenario(scenario, judge).await?);
        }
    }

    let passed_count = scenarios
        .iter()
        .filter(|scenario| scenario.status == ScenarioStatus::Passed)
        .count();
    let deterministic_failure_count = scenarios
        .iter()
        .filter(|scenario| !scenario.deterministic.passed)
        .count();
    let judge_failure_count = scenarios
        .iter()
        .filter(|scenario| scenario.judge.attempted && !scenario.judge.passed)
        .count();

    let report = EvalReport {
        schema_version: REPORT_SCHEMA_VERSION.to_string(),
        generated_at: OffsetDateTime::now_utc()
            .format(&time::format_description::well_known::Rfc3339)?,
        summary: EvalSummary {
            scenario_count: scenarios.len(),
            passed_count,
            deterministic_failure_count,
            judge_failure_count,
        },
        scenarios,
    };

    if report
        .scenarios
        .iter()
        .any(|scenario| !scenario.deterministic.passed)
    {
        return Ok(report);
    }

    if options.fail_on_judge_regression
        && report
            .scenarios
            .iter()
            .any(|scenario| scenario.judge.attempted && !scenario.judge.passed)
    {
        return Ok(report);
    }

    Ok(report)
}

pub fn report_exit_code(report: &EvalReport, options: &EvalRunOptions) -> EvalExitCode {
    if report
        .scenarios
        .iter()
        .any(|scenario| !scenario.deterministic.passed)
    {
        return EvalExitCode::DeterministicFailure;
    }
    if report.scenarios.iter().any(|scenario| {
        !scenario.judge.passed
            && scenario.judge.mode != JudgeMode::Disabled
            && (options.fail_on_judge_regression || scenario.judge.enforce_failure_policy)
    }) {
        return EvalExitCode::JudgeRegression;
    }
    EvalExitCode::Success
}

pub fn write_report(path: &Path, report: &EvalReport) -> Result<()> {
    let body = serde_json::to_string_pretty(report)?;
    fs::write(path, body).with_context(|| format!("write report {}", path.display()))?;
    Ok(())
}

pub fn build_router_judge_from_models_dir(models_dir: &Path) -> Result<Option<RouterJudge>> {
    let profiles = load_model_profiles(models_dir)?;
    let routing = load_routing(models_dir.join("routing.toml"))?;
    let default_profile = routing.profile_for_task("judge").map(ToString::to_string);
    let registry = build_registry(&profiles);
    if registry.profile_ids().is_empty() {
        return Ok(None);
    }
    Ok(Some(RouterJudge::new(
        Router::new(registry),
        default_profile,
    )))
}

pub fn default_models_dir() -> PathBuf {
    std::env::var("VEL_MODELS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(DEFAULT_MODELS_DIR))
}

async fn run_scenario(
    scenario: &EvalScenarioFixture,
    judge: Option<&dyn JudgeExecutor>,
) -> Result<ScenarioReport> {
    let replay = replay_day_scenario(scenario.kind, &scenario.simulation)
        .await
        .map_err(|error| anyhow!(error.to_string()))?;
    let deterministic = evaluate_deterministic_expectations(&replay, &scenario.expectations);
    let judge_report = if deterministic.passed {
        run_judge(scenario, &replay, judge).await
    } else {
        JudgeReport {
            mode: scenario.judge.mode.clone(),
            attempted: false,
            passed: false,
            enforce_failure_policy: scenario.judge.fail_on_regression,
            skipped_reason: Some("deterministic checks failed".to_string()),
            score: None,
            threshold: scenario.judge.pass_score,
            summary: None,
            reasons: Vec::new(),
        }
    };
    let status = if deterministic.passed && (!judge_report.attempted || judge_report.passed) {
        ScenarioStatus::Passed
    } else {
        ScenarioStatus::Failed
    };

    Ok(ScenarioReport {
        name: scenario.name.clone(),
        status,
        deterministic: DeterministicReport {
            passed: deterministic.passed,
            failures: deterministic.failures,
            replay: Some(replay),
        },
        judge: judge_report,
    })
}

struct DeterministicOutcome {
    passed: bool,
    failures: Vec<String>,
}

fn evaluate_deterministic_expectations(
    replay: &ReplayReport,
    expectations: &DeterministicExpectations,
) -> DeterministicOutcome {
    let mut failures = Vec::new();
    if let Some(expected_date) = expectations.expected_date.as_deref() {
        let actual = replay
            .context_json
            .get("date")
            .and_then(serde_json::Value::as_str);
        if actual != Some(expected_date) {
            failures.push(format!(
                "expected date {}, got {}",
                expected_date,
                actual.unwrap_or("<missing>")
            ));
        }
    }
    for required in &expectations.required_events {
        let found = replay
            .boundary_events
            .iter()
            .any(|event| event.event_type.to_string() == *required);
        if !found {
            failures.push(format!("missing required event {}", required));
        }
    }
    if let Some(min_ref_count) = expectations.min_ref_count {
        if replay.ref_count < min_ref_count {
            failures.push(format!(
                "expected at least {} refs, got {}",
                min_ref_count, replay.ref_count
            ));
        }
    }

    DeterministicOutcome {
        passed: failures.is_empty(),
        failures,
    }
}

async fn run_judge(
    scenario: &EvalScenarioFixture,
    replay: &ReplayReport,
    judge: Option<&dyn JudgeExecutor>,
) -> JudgeReport {
    match scenario.judge.mode {
        JudgeMode::Disabled => JudgeReport {
            mode: JudgeMode::Disabled,
            attempted: false,
            passed: true,
            enforce_failure_policy: false,
            skipped_reason: Some("judge disabled".to_string()),
            score: None,
            threshold: scenario.judge.pass_score,
            summary: None,
            reasons: Vec::new(),
        },
        JudgeMode::Router => {
            let Some(judge) = judge else {
                return JudgeReport {
                    mode: JudgeMode::Router,
                    attempted: false,
                    passed: false,
                    enforce_failure_policy: scenario.judge.fail_on_regression,
                    skipped_reason: Some("judge requested but no router is available".to_string()),
                    score: None,
                    threshold: scenario.judge.pass_score,
                    summary: None,
                    reasons: Vec::new(),
                };
            };
            match judge.evaluate(scenario, replay, &scenario.judge).await {
                Ok(decision) => {
                    let threshold = scenario.judge.pass_score.unwrap_or(0.7);
                    JudgeReport {
                        mode: JudgeMode::Router,
                        attempted: true,
                        passed: decision.score >= threshold,
                        enforce_failure_policy: scenario.judge.fail_on_regression,
                        skipped_reason: None,
                        score: Some(decision.score),
                        threshold: Some(threshold),
                        summary: Some(decision.summary),
                        reasons: decision.reasons,
                    }
                }
                Err(error) => JudgeReport {
                    mode: JudgeMode::Router,
                    attempted: true,
                    passed: false,
                    enforce_failure_policy: scenario.judge.fail_on_regression,
                    skipped_reason: Some(error.to_string()),
                    score: None,
                    threshold: scenario.judge.pass_score,
                    summary: None,
                    reasons: Vec::new(),
                },
            }
        }
    }
}

fn build_registry(profiles: &[ModelProfile]) -> ProviderRegistry {
    let mut registry = ProviderRegistry::default();
    for profile in profiles {
        if !profile.enabled {
            continue;
        }
        match profile.provider.as_str() {
            "llama_cpp" => {
                let provider = LlamaCppProvider::new(LlamaCppConfig {
                    base_url: profile.base_url.clone(),
                    model_id: profile.model.clone(),
                    context_window: profile.context_window,
                    supports_tools: profile.supports_tools,
                    supports_json: profile.supports_json,
                });
                registry.register(profile.id.clone(), Arc::new(provider));
            }
            "openai_oauth" => {
                if !is_local_host(&profile.base_url) {
                    continue;
                }
                let provider = OpenAiOauthProvider::new(OpenAiOauthConfig {
                    base_url: profile.base_url.clone(),
                    model_id: profile.model.clone(),
                    context_window: profile.context_window,
                    supports_tools: profile.supports_tools,
                    supports_json: profile.supports_json,
                });
                registry.register(profile.id.clone(), Arc::new(provider));
            }
            _ => {}
        }
    }
    registry
}

fn is_local_host(base_url: &str) -> bool {
    let host = match reqwest::Url::parse(base_url) {
        Ok(url) => url.host_str().map(ToString::to_string),
        Err(_) => None,
    };
    matches!(host.as_deref(), Some("localhost") | Some("127.0.0.1"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use vel_core::PrivacyClass;
    use vel_sim::{CaptureFixture, SignalFixture};

    fn sample_scenario() -> EvalScenarioFixture {
        let now = time::macros::datetime!(2026-03-18 15:30:00 UTC);
        EvalScenarioFixture {
            name: "today-context".to_string(),
            kind: ScenarioKind::Today,
            simulation: DayScenarioFixture {
                now,
                timezone: Some("UTC".to_string()),
                captures: vec![CaptureFixture {
                    content_text: "follow up on the budget review".to_string(),
                    capture_type: "note".to_string(),
                    occurred_at: now.unix_timestamp() - 600,
                    source_device: Some("laptop".to_string()),
                    privacy_class: PrivacyClass::Private,
                }],
                signals: vec![SignalFixture {
                    signal_type: "external_task".to_string(),
                    source: "todoist".to_string(),
                    source_ref: Some("task-1".to_string()),
                    timestamp: now.unix_timestamp() - 300,
                    payload_json: serde_json::json!({"text": "follow up on the budget review"}),
                }],
            },
            expectations: DeterministicExpectations {
                expected_date: Some("2026-03-18".to_string()),
                required_events: vec![
                    "run_created".to_string(),
                    "run_started".to_string(),
                    "context_generated".to_string(),
                    "artifact_written".to_string(),
                    "refs_created".to_string(),
                    "run_succeeded".to_string(),
                ],
                min_ref_count: Some(2),
            },
            judge: JudgeConfig::default(),
        }
    }

    struct FakeJudge {
        score: f32,
    }

    #[async_trait]
    impl JudgeExecutor for FakeJudge {
        async fn evaluate(
            &self,
            _scenario: &EvalScenarioFixture,
            _replay: &ReplayReport,
            _config: &JudgeConfig,
        ) -> Result<JudgeDecision> {
            Ok(JudgeDecision {
                score: self.score,
                summary: "good".to_string(),
                reasons: vec!["grounded".to_string()],
            })
        }
    }

    #[tokio::test]
    async fn deterministic_failures_prevent_pass() {
        let mut scenario = sample_scenario();
        scenario.expectations.expected_date = Some("2026-03-17".to_string());

        let report = run_fixture_sets(
            &[EvalFixtureSet {
                schema_version: FIXTURE_SCHEMA_VERSION.to_string(),
                scenarios: vec![scenario],
            }],
            Some(&FakeJudge { score: 0.95 }),
            &EvalRunOptions {
                fail_on_judge_regression: true,
            },
        )
        .await
        .expect("report should build");

        assert_eq!(report.summary.deterministic_failure_count, 1);
        assert_eq!(
            report_exit_code(
                &report,
                &EvalRunOptions {
                    fail_on_judge_regression: true
                }
            ),
            EvalExitCode::DeterministicFailure
        );
        assert!(!report.scenarios[0].judge.attempted);
    }

    #[tokio::test]
    async fn judge_regression_can_fail_exit_policy() {
        let mut scenario = sample_scenario();
        scenario.judge = JudgeConfig {
            mode: JudgeMode::Router,
            model_profile: Some("judge".to_string()),
            rubric: vec!["be grounded".to_string()],
            pass_score: Some(0.8),
            fail_on_regression: true,
        };

        let report = run_fixture_sets(
            &[EvalFixtureSet {
                schema_version: FIXTURE_SCHEMA_VERSION.to_string(),
                scenarios: vec![scenario],
            }],
            Some(&FakeJudge { score: 0.6 }),
            &EvalRunOptions {
                fail_on_judge_regression: true,
            },
        )
        .await
        .expect("report should build");

        assert_eq!(report.summary.judge_failure_count, 1);
        assert_eq!(
            report_exit_code(
                &report,
                &EvalRunOptions {
                    fail_on_judge_regression: true
                }
            ),
            EvalExitCode::JudgeRegression
        );
    }

    #[test]
    fn fixture_file_rejects_unknown_schema_version() {
        let dir = std::env::temp_dir().join(format!(
            "veld_eval_fixture_{}",
            uuid::Uuid::new_v4().simple()
        ));
        fs::create_dir_all(&dir).expect("temp dir should exist");
        let path = dir.join("fixture.json");
        fs::write(
            &path,
            serde_json::json!({
                "schema_version": "wrong",
                "scenarios": [],
            })
            .to_string(),
        )
        .expect("fixture should write");

        let error = load_fixture_sets(&path).expect_err("schema mismatch should fail");
        assert!(error.to_string().contains("unsupported schema_version"));

        let _ = fs::remove_dir_all(&dir);
    }
}
