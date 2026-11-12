//! Port: turning the app's server off and on, over its state.

use async_trait::async_trait;

use super::server_state_service::ServerStateService;

/// Stops and starts the server the app runs, and tells where it is.
#[async_trait]
pub trait ServerSwitchService: ServerStateService {
    /// Starts the server with the settings as they are now.
    async fn turn_on(&self);
    /// Stops the server; its state becomes `Off`.
    async fn turn_off(&self);
}

#[cfg(test)]
mockall::mock! {
    /// A server's state and switch.
    pub ServerSwitchService {}

    #[async_trait]
    impl ServerStateService for ServerSwitchService {
        async fn current(&self) -> crate::core::domain::server_state_enum::ServerStateEnum;
    }

    #[async_trait]
    impl ServerSwitchService for ServerSwitchService {
        async fn turn_on(&self);
        async fn turn_off(&self);
    }
}

#[cfg(test)]
#[path = "server_switch_service_test.rs"]
mod server_switch_service_test;
