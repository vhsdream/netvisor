use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct OpenWebUI;

impl ServiceDefinition for OpenWebUI {
    fn name(&self) -> &'static str {
        "Open WebUI"
    }
    fn description(&self) -> &'static str {
        "Open, extensible, user-friendly interface for AI"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Web
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortBase::HttpAlt, "/manifest.json", "Open WebUI")
    }

    fn logo_url(&self) -> &'static str {
        "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/svg/open-webui-light.svg"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<OpenWebUI>));
