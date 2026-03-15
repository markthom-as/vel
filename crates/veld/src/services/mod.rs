//! Application service layer. Routes call these; services contain logic and use storage/config.

pub mod context_generation;
pub mod context_runs;
pub mod doctor;
pub mod inference;
pub mod nudge_engine;
pub mod risk;
pub mod synthesis;
