use vel_core::{SynthesisFailure, SynthesisFailureKind, SynthesisRequest, SynthesisResponse};

pub trait HarnessLlm {
    async fn synthesize(
        &self,
        request: &SynthesisRequest,
    ) -> Result<SynthesisResponse, SynthesisFailure>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct MockHarnessLlm;

impl HarnessLlm for MockHarnessLlm {
    async fn synthesize(
        &self,
        request: &SynthesisRequest,
    ) -> Result<SynthesisResponse, SynthesisFailure> {
        let intent = request.intent.trim();
        if intent.is_empty() {
            return Err(SynthesisFailure {
                kind: SynthesisFailureKind::InvalidResponse,
                message: "intent must not be empty".to_string(),
            });
        }
        Ok(SynthesisResponse {
            plan_steps: vec![
                format!("interpret intent: {intent}"),
                "assemble context from persisted records".to_string(),
                "propose execution steps and cautions".to_string(),
            ],
            rationale: "deterministic mock synthesis".to_string(),
            cautions: vec!["mock adapter used".to_string()],
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DisabledHarnessLlm;

impl HarnessLlm for DisabledHarnessLlm {
    async fn synthesize(
        &self,
        _request: &SynthesisRequest,
    ) -> Result<SynthesisResponse, SynthesisFailure> {
        Err(SynthesisFailure {
            kind: SynthesisFailureKind::ProviderUnavailable,
            message: "no harness llm provider configured".to_string(),
        })
    }
}

pub enum HarnessLlmAdapter {
    Mock(MockHarnessLlm),
    Disabled(DisabledHarnessLlm),
}

impl HarnessLlmAdapter {
    pub fn from_selection(selection: &str) -> Self {
        match selection.trim() {
            "mock" => Self::Mock(MockHarnessLlm),
            _ => Self::Disabled(DisabledHarnessLlm),
        }
    }
}

impl HarnessLlm for HarnessLlmAdapter {
    async fn synthesize(
        &self,
        request: &SynthesisRequest,
    ) -> Result<SynthesisResponse, SynthesisFailure> {
        match self {
            Self::Mock(adapter) => adapter.synthesize(request).await,
            Self::Disabled(adapter) => adapter.synthesize(request).await,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{HarnessLlm, HarnessLlmAdapter, MockHarnessLlm};
    use vel_core::SynthesisFailureKind;

    #[tokio::test]
    async fn mock_adapter_is_deterministic() {
        let adapter = MockHarnessLlm;
        let request = vel_core::SynthesisRequest {
            intent: "summarize week".to_string(),
            context_json: serde_json::json!({}),
        };
        let first = adapter.synthesize(&request).await.expect("first");
        let second = adapter.synthesize(&request).await.expect("second");
        assert_eq!(first, second);
    }

    #[tokio::test]
    async fn disabled_adapter_surfaces_canonical_failure() {
        let adapter = HarnessLlmAdapter::from_selection("unsupported");
        let error = adapter
            .synthesize(&vel_core::SynthesisRequest {
                intent: "summarize".to_string(),
                context_json: serde_json::json!({}),
            })
            .await
            .expect_err("must fail");
        assert_eq!(error.kind, SynthesisFailureKind::ProviderUnavailable);
    }
}
