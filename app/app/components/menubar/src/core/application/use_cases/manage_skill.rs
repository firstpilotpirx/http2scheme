//! Input port: one request of the Agents tab.

use async_trait::async_trait;

use crate::core::domain::agents_command_enum::AgentsCommandEnum;
use crate::core::domain::agents_view_value_object::AgentsViewValueObject;

/// Runs one Agents tab request and answers what the tab shows next.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ManageSkill: Send + Sync {
    /// Runs the use case; a failure is shown in the view, not returned.
    async fn execute(&self, command: AgentsCommandEnum) -> AgentsViewValueObject;
}

#[cfg(test)]
#[path = "manage_skill_test.rs"]
mod manage_skill_test;
