use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Karakeep;

impl ServiceDefinition for Karakeep {
    fn name(&self) -> &'static str {
        "Karakeep"
    }
    fn description(&self) -> &'static str {
        "The Bookmark Everything App"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Media
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortBase::new_tcp(3000), "/signin", "Karakeep")
    }

    fn logo_url(&self) -> &'static str {
        "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/svg/karakeep.svg"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Karakeep>));
