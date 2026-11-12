//! Use cases: one input trait and one struct each.

pub mod check_agent_status;
pub mod check_agent_status_use_case;
pub mod install_agent;
pub mod install_agent_use_case;
pub mod open_link;
pub mod open_link_command;
pub mod open_link_use_case;
pub mod prepare_link;
pub mod prepare_link_use_case;
pub mod prepare_settings;
pub mod prepare_settings_use_case;
pub mod replay_link;
pub mod replay_link_use_case;
pub mod start_agent;
pub mod start_agent_use_case;
pub mod stop_agent;
pub mod stop_agent_use_case;
pub mod uninstall_agent;
pub mod uninstall_agent_use_case;

#[cfg(test)]
mod agent_fixtures_test;
