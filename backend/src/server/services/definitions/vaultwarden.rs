use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Vaultwarden;

impl ServiceDefinition for Vaultwarden {
    fn name(&self) -> &'static str {
        "Vaultwarden"
    }
    fn description(&self) -> &'static str {
        "Self-hosted Bitwarden-compatible server, written in Rust"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Web
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortBase::Http, "/", "vaultwarden web")
    }

    fn logo_url(&self) -> &'static str {
        "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/svg/vaultwarden.svg"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Vaultwarden>));
