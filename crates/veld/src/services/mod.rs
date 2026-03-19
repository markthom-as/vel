//! Application service layer. Routes call these; services contain logic and use storage/config.
//!
//! **Read vs evaluate boundary:** Explain and read routes use only storage (read-only). The only
//! entry point for recompute-and-persist is [evaluate::run]. See docs/tickets/repo-feedback/001.

pub mod adaptive_policies;
pub mod agent_protocol;
pub mod apple_behavior;
pub mod apple_voice;
pub mod broker;
pub mod chat;
pub mod client_sync;
pub mod command_lang;
pub mod components;
pub mod connect_runtime;
pub mod context_generation;
pub mod context_runs;
pub mod doctor;
pub mod evaluate;
pub mod execution_context;
pub mod explain;
pub mod inference;
pub(crate) mod integration_runtime;
pub mod integrations;
pub(crate) mod integrations_email;
pub(crate) mod integrations_github;
pub(crate) mod integrations_google;
pub(crate) mod integrations_host;
pub(crate) mod integrations_todoist;
pub mod journal;
pub(crate) mod lan_discovery;
pub mod linking;
pub(crate) mod local_network;
pub mod now;
pub mod nudge_engine;
pub mod operator_queue;
pub(crate) mod operator_settings;
pub mod people;
pub mod projects;
pub mod retrieval;
pub mod risk;
pub mod sandbox;
pub mod suggestions;
pub mod synthesis;
pub(crate) mod tailscale;
pub mod timezone;
pub mod uncertainty;
pub mod writeback;
