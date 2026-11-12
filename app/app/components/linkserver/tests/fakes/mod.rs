//! Fake secondary adapters for the slice scenarios.

use std::sync::Mutex;

use async_trait::async_trait;
use linkserver::core::application::exceptions::open_link_error::OpenLinkError;
use linkserver::core::application::repositories::open_attempt_repository::OpenAttemptRepository;
use linkserver::core::application::services::browser_tab_closer_service::BrowserTabCloserService;
use linkserver::core::application::services::scheme_link_opener_service::SchemeLinkOpenerService;
use linkserver::core::domain::link_open_attempted_domain_event::LinkOpenAttemptedDomainEvent;
use linkserver::core::domain::port_value_object::PortValueObject;
use linkserver::core::domain::scheme_link_value_object::SchemeLinkValueObject;

fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Records every link it is asked to open; fails when told to.
#[derive(Debug, Default)]
pub struct FakeOpener {
    fails: bool,
    links: Mutex<Vec<String>>,
}

impl FakeOpener {
    pub fn failing() -> Self {
        Self {
            fails: true,
            ..Self::default()
        }
    }

    pub fn links(&self) -> Vec<String> {
        lock(&self.links).clone()
    }
}

#[async_trait]
impl SchemeLinkOpenerService for FakeOpener {
    async fn open(&self, link: &SchemeLinkValueObject) -> Result<(), OpenLinkError> {
        lock(&self.links).push(link.as_str().to_owned());
        if self.fails {
            return Err(OpenLinkError::OpenFailed {
                reason: "no app".to_owned(),
            });
        }
        Ok(())
    }
}

/// Records every port whose tabs it is asked to close; fails when told to.
#[derive(Debug, Default)]
pub struct FakeCloser {
    fails: bool,
    ports: Mutex<Vec<u16>>,
}

impl FakeCloser {
    pub fn failing() -> Self {
        Self {
            fails: true,
            ..Self::default()
        }
    }

    pub fn ports(&self) -> Vec<u16> {
        lock(&self.ports).clone()
    }
}

#[async_trait]
impl BrowserTabCloserService for FakeCloser {
    async fn close_open_tabs(&self, port: PortValueObject) -> Result<(), OpenLinkError> {
        lock(&self.ports).push(port.value());
        if self.fails {
            return Err(OpenLinkError::TabCloseFailed {
                reason: "refused".to_owned(),
            });
        }
        Ok(())
    }
}

/// Keeps every recorded event in memory.
#[derive(Debug, Default)]
pub struct FakeLog {
    events: Mutex<Vec<LinkOpenAttemptedDomainEvent>>,
}

impl FakeLog {
    pub fn events(&self) -> Vec<LinkOpenAttemptedDomainEvent> {
        lock(&self.events).clone()
    }
}

#[async_trait]
impl OpenAttemptRepository for FakeLog {
    async fn record(&self, event: &LinkOpenAttemptedDomainEvent) -> Result<(), OpenLinkError> {
        lock(&self.events).push(event.clone());
        Ok(())
    }
}
