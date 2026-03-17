use vel_core::{ConfidenceBand, ResolutionMode};

pub fn band_from_score(score: f64) -> ConfidenceBand {
    if score < 0.45 {
        ConfidenceBand::Low
    } else if score < 0.75 {
        ConfidenceBand::Medium
    } else {
        ConfidenceBand::High
    }
}

pub fn resolution_mode_for(score: f64, decision_kind: &str) -> ResolutionMode {
    match decision_kind {
        "suggestion_generation" if score < 0.45 => ResolutionMode::Defer,
        _ if score < 0.25 => ResolutionMode::SilentHold,
        _ if score < 0.45 => ResolutionMode::AskUser,
        _ => ResolutionMode::Proceed,
    }
}
