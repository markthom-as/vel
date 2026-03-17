//! Application service layer. Routes call these; services contain logic and use storage/config.
//!
//! **Read vs evaluate boundary:** Explain and read routes use only storage (read-only). The only
//! entry point for recompute-and-persist is [evaluate::run]. See docs/tickets/repo-feedback/001.

pub mod components;
pub mod context_generation;
pub mod context_runs;
pub mod doctor;
pub mod evaluate;
pub mod inference;
pub mod integrations;
pub mod nudge_engine;
pub mod risk;
pub mod suggestions;
pub mod synthesis;
