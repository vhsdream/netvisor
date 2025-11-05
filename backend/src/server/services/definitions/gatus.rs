use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Gatus;

impl ServiceDefinition for Gatus {
    fn name(&self) -> &'static str {
        "Gatus"
    }
    fn description(&self) -> &'static str {
        "Automated developer-oriented status page"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortBase::Http, "/manifest.json", "Gatus")
    }

    fn dashboard_icons_path(&self) -> &'static str {
        "gatus"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Gatus>));
