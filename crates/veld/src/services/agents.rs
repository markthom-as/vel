use time::OffsetDateTime;
use vel_api_types::{
    AgentBudgetsData, AgentMemoryScopeData, AgentReturnContractData, AgentReturnEvidenceData,
    AgentReturnedArtifactData, AgentRuntimeViewData, AgentSpecData, AgentSuggestedActionData,
};
use vel_config::{AgentBudgets as ConfigAgentBudgets, AgentMemoryScope as ConfigAgentMemoryScope};
use vel_config::{AgentSpec as ConfigAgentSpec, AppConfig};
use vel_core::{
    AgentPriority, AgentReturnContract, AgentRunStatus, AgentSpawnRequest, AgentSpec, RunId,
    RunKind,
};
use vel_storage::{AgentRunInsert, Storage};

use crate::errors::AppError;

pub fn list_specs(config: &AppConfig) -> Result<Vec<AgentSpec>, AppError> {
    config
        .load_agent_specs()
        .map(|specs| specs.into_iter().map(map_config_spec).collect())
        .map_err(|error| AppError::internal(format!("failed to load agent specs: {}", error)))
}

pub fn list_spec_data(config: &AppConfig) -> Result<Vec<AgentSpecData>, AppError> {
    list_specs(config)?
        .into_iter()
        .map(|spec| map_spec_data(&spec))
        .collect::<Vec<_>>()
        .pipe(Ok)
}

pub fn get_spec(config: &AppConfig, agent_id: &str) -> Result<AgentSpec, AppError> {
    list_specs(config)?
        .into_iter()
        .find(|spec| spec.id == agent_id)
        .ok_or_else(|| AppError::not_found("agent spec not found"))
}

pub async fn spawn_run(
    storage: &Storage,
    config: &AppConfig,
    request: AgentSpawnRequest,
) -> Result<AgentRuntimeViewData, AppError> {
    let spec = get_spec(config, &request.agent_id)?;
    request
        .validate_for_spec(&spec)
        .map_err(|error| AppError::bad_request(error.to_string()))?;

    if !request.mission_input.is_object() {
        return Err(AppError::bad_request(
            "mission_input must be a JSON object for agent runs",
        ));
    }

    let now = OffsetDateTime::now_utc();
    if let Some(deadline) = request.deadline {
        if deadline <= now {
            return Err(AppError::bad_request("deadline must be in the future"));
        }
    }

    if let Some(parent_run_id) = &request.parent_run_id {
        if storage
            .get_run_by_id(parent_run_id.as_ref())
            .await?
            .is_none()
        {
            return Err(AppError::bad_request("parent_run_id does not exist"));
        }
    }

    let run_id = RunId::new();
    let expires_at = request
        .deadline
        .unwrap_or(now + time::Duration::seconds(spec.ttl_seconds as i64));
    let input_json = serde_json::json!({
        "agent_id": spec.id,
        "mission": spec.mission,
        "mission_input": request.mission_input,
        "parent_run_id": request.parent_run_id.as_ref().map(AsRef::as_ref),
        "deadline": request.deadline.map(|value| value.unix_timestamp()),
        "priority": request.priority.map(agent_priority_string),
        "ttl_seconds": spec.ttl_seconds,
        "kind": spec.kind.to_string(),
        "allowed_tools": spec.allowed_tools,
        "memory_scope": spec.memory_scope,
        "return_contract": spec.return_contract,
        "budgets": spec.budgets,
        "side_effect_policy": spec.side_effect_policy,
    });

    storage
        .create_run(&run_id, RunKind::Agent, &input_json)
        .await?;
    storage
        .create_agent_run(AgentRunInsert {
            run_id: run_id.as_ref().to_string(),
            agent_id: spec.id.clone(),
            parent_run_id: request.parent_run_id.as_ref().map(ToString::to_string),
            status: AgentRunStatus::Queued,
            mission_input: request.mission_input,
            deadline_ts: request.deadline.map(|value| value.unix_timestamp()),
            ttl_seconds: spec.ttl_seconds,
            expires_at: expires_at.unix_timestamp(),
            waiting_reason: None,
            return_contract: spec.return_contract.clone(),
            max_tool_calls: spec.budgets.max_tool_calls,
            max_tokens: spec.budgets.max_tokens,
            allowed_tools: spec.allowed_tools.clone(),
            memory_scope: spec.memory_scope.clone(),
        })
        .await?;

    get_run_view(storage, config, run_id.as_ref()).await
}

pub async fn get_run_view(
    storage: &Storage,
    config: &AppConfig,
    run_id: &str,
) -> Result<AgentRuntimeViewData, AppError> {
    let agent_run = storage
        .get_agent_run(run_id)
        .await?
        .ok_or_else(|| AppError::not_found("agent run not found"))?;
    let run = storage
        .get_run_by_id(run_id)
        .await?
        .ok_or_else(|| AppError::not_found("run not found"))?;
    let spec = get_spec(config, &agent_run.agent_id)?;

    Ok(AgentRuntimeViewData {
        run_id: run.id,
        spec_id: agent_run.agent_id,
        status: agent_run.status.to_string(),
        kind: spec.kind.to_string(),
        mission_input: agent_run.mission_input,
        parent_run_id: agent_run.parent_run_id.map(RunId::from),
        deadline: agent_run
            .deadline_ts
            .and_then(|ts| OffsetDateTime::from_unix_timestamp(ts).ok()),
        priority: run
            .input_json
            .get("priority")
            .and_then(serde_json::Value::as_str)
            .map(ToString::to_string),
        created_at: run.created_at,
        started_at: run.started_at,
        finished_at: run.finished_at,
        waiting_reason: agent_run.waiting_reason,
        summary: agent_run.summary,
        confidence: agent_run.confidence,
        output: run.output_json,
        return_contract: agent_run.structured_output.map(map_return_contract_data),
    })
}

fn map_config_spec(spec: ConfigAgentSpec) -> AgentSpec {
    AgentSpec {
        id: spec.id,
        mission: spec.mission,
        kind: match spec.kind {
            vel_config::AgentSpecKind::Subagent => vel_core::AgentKind::Subagent,
            vel_config::AgentSpecKind::Supervisor => vel_core::AgentKind::Supervisor,
            vel_config::AgentSpecKind::Specialist => vel_core::AgentKind::Specialist,
        },
        allowed_tools: spec.allowed_tools,
        memory_scope: map_memory_scope(spec.memory_scope),
        return_contract: spec.return_contract,
        ttl_seconds: spec.ttl_seconds,
        budgets: map_budgets(spec.budgets),
        mission_input_schema: spec.mission_input_schema,
        side_effect_policy: spec.side_effect_policy,
    }
}

fn map_spec_data(spec: &AgentSpec) -> AgentSpecData {
    AgentSpecData {
        id: spec.id.clone(),
        mission: spec.mission.clone(),
        kind: spec.kind.to_string(),
        allowed_tools: spec.allowed_tools.clone(),
        memory_scope: AgentMemoryScopeData {
            constitution: spec.memory_scope.constitution,
            topic_pads: spec.memory_scope.topic_pads.clone(),
            event_query: spec.memory_scope.event_query.clone(),
        },
        return_contract: spec.return_contract.clone(),
        ttl_seconds: spec.ttl_seconds,
        budgets: AgentBudgetsData {
            max_tool_calls: spec.budgets.max_tool_calls,
            max_tokens: spec.budgets.max_tokens,
            max_memory_queries: spec.budgets.max_memory_queries,
            max_side_effects: spec.budgets.max_side_effects,
        },
        mission_input_schema: spec.mission_input_schema.clone(),
        side_effect_policy: spec.side_effect_policy.clone(),
    }
}

fn map_memory_scope(scope: ConfigAgentMemoryScope) -> vel_core::AgentMemoryScope {
    vel_core::AgentMemoryScope {
        constitution: scope.constitution,
        topic_pads: scope.topic_pads,
        event_query: scope.event_query,
    }
}

fn map_budgets(budgets: ConfigAgentBudgets) -> vel_core::AgentBudgets {
    vel_core::AgentBudgets {
        max_tool_calls: budgets.max_tool_calls,
        max_tokens: budgets.max_tokens,
        max_memory_queries: budgets.max_memory_queries,
        max_side_effects: budgets.max_side_effects,
    }
}

fn map_return_contract_data(contract: AgentReturnContract) -> AgentReturnContractData {
    AgentReturnContractData {
        status: match contract.status {
            vel_core::AgentReturnStatus::Waiting => "waiting",
            vel_core::AgentReturnStatus::Completed => "completed",
            vel_core::AgentReturnStatus::Failed => "failed",
            vel_core::AgentReturnStatus::Expired => "expired",
            vel_core::AgentReturnStatus::Cancelled => "cancelled",
        }
        .to_string(),
        summary: contract.summary,
        evidence: contract
            .evidence
            .into_iter()
            .map(|item| AgentReturnEvidenceData {
                kind: item.kind,
                value: item.value,
            })
            .collect(),
        confidence: contract.confidence,
        suggested_actions: contract
            .suggested_actions
            .into_iter()
            .map(|item| AgentSuggestedActionData {
                action_type: item.action_type,
                reason: item.reason,
            })
            .collect(),
        artifacts: contract
            .artifacts
            .into_iter()
            .map(|item| AgentReturnedArtifactData {
                artifact_type: item.artifact_type,
                location: item.location,
            })
            .collect(),
        errors: contract.errors,
    }
}

fn agent_priority_string(priority: AgentPriority) -> &'static str {
    match priority {
        AgentPriority::Low => "low",
        AgentPriority::Normal => "normal",
        AgentPriority::High => "high",
        AgentPriority::Urgent => "urgent",
    }
}

trait Pipe: Sized {
    fn pipe<T>(self, f: impl FnOnce(Self) -> T) -> T {
        f(self)
    }
}

impl<T> Pipe for T {}
