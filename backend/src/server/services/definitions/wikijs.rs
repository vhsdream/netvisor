use crate::server::hosts::r#impl::ports::PortBase;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct WikiJS;

impl ServiceDefinition for WikiJS {
    fn name(&self) -> &'static str {
        "WikiJS"
    }
    fn description(&self) -> &'static str {
        "A modern and powerful wiki app built on Node.js"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Collaboration
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortBase::Http3000, "/_app/manifest.json", "Wiki.js", None)
    }
    fn logo_url(&self) -> &'static str {
        "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/svg/wikijs.svg"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<WikiJS>));
