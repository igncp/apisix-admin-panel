use super::common::{PluginDefinition, PluginEntities, PluginOption};
use crate::{apisix::base::PropertyType, macros::derive_common_default};

derive_common_default! {
pub struct ConsumerRestrictionPlugin(PluginDefinition);}

impl ConsumerRestrictionPlugin {
    pub fn new() -> Self {
        Self(PluginDefinition {
            name: "consumer-restriction".to_string(),
            entities: [
                PluginEntities::Route,
                PluginEntities::Service,
                PluginEntities::Consumer,
                PluginEntities::ConsumerGroup,
            ]
            .iter()
            .cloned()
            .collect(),
            options: vec![
                PluginOption {
                    description: "Type of object to base the restriction on.".to_string(),
                    name: "type".to_string(),
                    property_type: PropertyType::Enum(vec![
                        "consumer_name".to_string(),
                        "consumer_group_id".to_string(),
                        "service_id".to_string(),
                        "route_id".to_string(),
                    ]),
                    ..Default::default()
                },
                PluginOption {
                    description: "List of objects to whitelist. Has a higher priority than allowed_by_methods.".to_string(),
                    name: "whitelist".to_string(),
                    property_type: PropertyType::List(Box::new(PropertyType::String)),
                    ..Default::default()
                },
                PluginOption {
                    description: "List of objects to blacklist. Has a higher priority than whitelist."
                        .to_string(),
                    name: "blacklist".to_string(),
                    property_type: PropertyType::List(Box::new(PropertyType::String)),
                    ..Default::default()
                },
                PluginOption {
                    description: "HTTP status code returned when the request is rejected."
                        .to_string(),
                    name: "rejected_code".to_string(),
                    property_type: PropertyType::Number,
                    ..Default::default()
                },
                PluginOption {
                    description: "Message returned when the request is rejected."
                        .to_string(),
                    name: "rejected_msg".to_string(),
                    property_type: PropertyType::String,
                    ..Default::default()
                },
                PluginOption {
                    description: "List of allowed configurations for Consumer settings, including a username of the Consumer and a list of allowed HTTP methods."
                        .to_string(),
                    name: "allowed_by_methods".to_string(),
                    property_type: PropertyType::List(Box::new(
                            PropertyType::JSON
                    )),
                    ..Default::default()
                },
            ],
        })
    }
}
