use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Slskd;

impl ServiceDefinition for Slskd {
    fn name(&self) -> &'static str {
        "Slskd"
    }
    fn description(&self) -> &'static str {
        "A modern client-server application for the Soulseek file-sharing network"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Media
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortBase::new_tcp(5030), "/", "slskd")
    }

    fn dashboard_icons_path(&self) -> &'static str {
        "slskd"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Slskd>));
