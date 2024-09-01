use super::common::{PluginDefinition, PluginEntities, PluginOption};
use crate::{
    apisix::base::{PropertyType, Required},
    macros::derive_common_default,
};

derive_common_default! {
pub struct UriBlockerPlugin(PluginDefinition);}

impl UriBlockerPlugin {
    pub fn new() -> Self {
        Self(PluginDefinition {
            name: "uri-blocker".to_string(),
            entities: [PluginEntities::Route, PluginEntities::Service]
                .iter()
                .cloned()
                .collect(),
            options: vec![
                PluginOption {
                    description: "List of regex filter rules. If the request URI hits any one of the rules, the response code is set to the rejected_code and the user request is terminated. ".to_string(),
                    name: "block_rules".to_string(),
                    is_required: Required::True,
                    property_type: PropertyType::List(
                        Box::new(PropertyType::String),
                    ),
                    ..Default::default()
                },
                PluginOption {
                    description: "HTTP status code returned when the request URI hits any of the block_rules.".to_string(),
                    name: "rejected_code".to_string(),
                    property_type: PropertyType::Number,
                    ..Default::default()
                },
                PluginOption {
                    description: "non-empty	HTTP response body returned when the request URI hits any of the block_rules.".to_string(),
                    name: "rejected_msg".to_string(),
                    ..Default::default()
                },
                PluginOption {
                    description: "When set to true, ignores the case when matching request URI.".to_string(),
                    name: "case_insensitive".to_string(),
                    property_type: PropertyType::Boolean,
                    ..Default::default()
                }
            ],
        })
    }
}
