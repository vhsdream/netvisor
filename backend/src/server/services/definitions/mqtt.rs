use crate::server::hosts::r#impl::ports::PortBase;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct WebService;

impl ServiceDefinition for WebService {
    fn name(&self) -> &'static str {
        "mqtt"
    }
    fn description(&self) -> &'static str {
        "A generic mqtt Service"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::MessageQueue
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AnyOf(vec![
            Pattern::Port(PortBase::new_tcp(1883)),
            Pattern::Port(PortBase::new_tcp(8883)),
        ])
    }

    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<WebService>));
