use crate::server::hosts::r#impl::ports::PortBase;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct BookLore;

impl ServiceDefinition for BookLore {
    fn name(&self) -> &'static str {
        "BookLore"
    }
    fn description(&self) -> &'static str {
        "A self-hosted, multi-user digital library"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Media
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::Endpoint(PortBase::new_tcp(6060), "/", "BookLore", None),
            Pattern::Endpoint(
                PortBase::new_tcp(6060),
                "/ap1/v1/setup/status",
                "Initial setup",
                None,
            ),
        ])
    }

    fn logo_url(&self) -> &'static str {
        "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/svg/booklore.svg"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<BookLore>));
