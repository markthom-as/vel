//! Application service layer. Routes call these; services contain logic and use storage/config.
//!
//! **Read vs evaluate boundary:** Explain and read routes use only storage (read-only). The only
//! entry point for recompute-and-persist is [evaluate::run]. See docs/tickets/repo-feedback/001.

pub mod adaptive_policies;
pub mod broker;
pub mod chat;
pub mod client_sync;
pub mod command_lang;
pub mod components;
pub mod context_generation;
pub mod context_runs;
pub mod doctor;
pub mod evaluate;
pub mod explain;
pub mod inference;
pub(crate) mod integration_runtime;
pub mod integrations;
pub(crate) mod integrations_google;
pub(crate) mod integrations_host;
pub(crate) mod integrations_todoist;
pub mod journal;
pub mod now;
pub mod nudge_engine;
pub(crate) mod operator_settings;
pub mod risk;
pub mod suggestions;
pub mod synthesis;
pub mod timezone;
pub mod uncertainty;
