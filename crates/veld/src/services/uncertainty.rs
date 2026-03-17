use vel_core::{AskBeforeActingMode, ConfidenceBand, ResolutionMode};
use vel_storage::Storage;

const ASK_BEFORE_ACTING_MODE_KEY: &str = "ask_before_acting_mode";

pub fn band_from_score(score: f64) -> ConfidenceBand {
    if score < 0.45 {
        ConfidenceBand::Low
    } else if score < 0.75 {
        ConfidenceBand::Medium
    } else {
        ConfidenceBand::High
    }
}

pub async fn ask_before_acting_mode(
    storage: &Storage,
) -> Result<AskBeforeActingMode, crate::errors::AppError> {
    let settings = storage.get_all_settings().await?;
    Ok(settings
        .get(ASK_BEFORE_ACTING_MODE_KEY)
        .and_then(|value| value.as_str())
        .and_then(|value| value.parse().ok())
        .unwrap_or_default())
}

pub fn resolution_mode_for(
    score: f64,
    decision_kind: &str,
    mode: AskBeforeActingMode,
) -> ResolutionMode {
    let thresholds = thresholds_for(mode);
    match decision_kind {
        "suggestion_generation" if score < thresholds.suggestion_defer_cutoff => {
            ResolutionMode::Defer
        }
        _ if score < thresholds.silent_hold_cutoff => ResolutionMode::SilentHold,
        _ if score < thresholds.ask_user_cutoff => ResolutionMode::AskUser,
        _ => ResolutionMode::Proceed,
    }
}

struct ClarificationThresholds {
    silent_hold_cutoff: f64,
    ask_user_cutoff: f64,
    suggestion_defer_cutoff: f64,
}

fn thresholds_for(mode: AskBeforeActingMode) -> ClarificationThresholds {
    match mode {
        AskBeforeActingMode::HandsOff => ClarificationThresholds {
            silent_hold_cutoff: 0.15,
            ask_user_cutoff: 0.35,
            suggestion_defer_cutoff: 0.35,
        },
        AskBeforeActingMode::Balanced => ClarificationThresholds {
            silent_hold_cutoff: 0.25,
            ask_user_cutoff: 0.45,
            suggestion_defer_cutoff: 0.45,
        },
        AskBeforeActingMode::HighClarity => ClarificationThresholds {
            silent_hold_cutoff: 0.35,
            ask_user_cutoff: 0.60,
            suggestion_defer_cutoff: 0.60,
        },
        AskBeforeActingMode::DelegateToAgentsFirst => ClarificationThresholds {
            silent_hold_cutoff: 0.25,
            ask_user_cutoff: 0.55,
            suggestion_defer_cutoff: 0.55,
        },
        AskBeforeActingMode::AskBeforeDestructiveActionsOnly => ClarificationThresholds {
            silent_hold_cutoff: 0.15,
            ask_user_cutoff: 0.30,
            suggestion_defer_cutoff: 0.30,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modes_shift_suggestion_thresholds() {
        assert_eq!(
            resolution_mode_for(0.42, "suggestion_generation", AskBeforeActingMode::HandsOff),
            ResolutionMode::Proceed
        );
        assert_eq!(
            resolution_mode_for(0.42, "suggestion_generation", AskBeforeActingMode::Balanced),
            ResolutionMode::Defer
        );
        assert_eq!(
            resolution_mode_for(
                0.42,
                "suggestion_generation",
                AskBeforeActingMode::HighClarity
            ),
            ResolutionMode::Defer
        );
    }

    #[test]
    fn modes_shift_general_interrupt_thresholds() {
        assert_eq!(
            resolution_mode_for(0.36, "generic", AskBeforeActingMode::HandsOff),
            ResolutionMode::Proceed
        );
        assert_eq!(
            resolution_mode_for(0.32, "generic", AskBeforeActingMode::Balanced),
            ResolutionMode::AskUser
        );
        assert_eq!(
            resolution_mode_for(0.20, "generic", AskBeforeActingMode::HighClarity),
            ResolutionMode::SilentHold
        );
    }
}
