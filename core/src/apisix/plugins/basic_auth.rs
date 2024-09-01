use super::common::{PluginDefinition, PluginEntities, PluginOption};
use crate::{
    apisix::base::{PropertyType, Required},
    macros::derive_common_default,
};

derive_common_default! {
pub struct ConsumerBasicAuthPlugin(PluginDefinition);}

impl ConsumerBasicAuthPlugin {
    pub fn new() -> Self {
        Self(PluginDefinition {
            name: "basic-auth".to_string(),
            entities: [PluginEntities::Consumer]
                .iter()
                .cloned()
                .collect(),
            options: vec![
                PluginOption {
                    default_value: None,
                    description: "Unique username for a Consumer. If multiple Consumers use the same username, a request matching exception is raised.".to_string(),
                    is_required: Required::True,
                    name: "username".to_string(),
                    property_type: PropertyType::String,
                },
                PluginOption {
                    default_value: None,
                    description: "Password of the user. This field supports saving the value in Secret Manager using the APISIX Secret resource.".to_string(),
                    is_required: Required::True,
                    name: "password".to_string(),
                    property_type: PropertyType::String,
                },
            ],
        })
    }
}

derive_common_default! {
pub struct RouteBasicAuthPlugin(PluginDefinition);}

impl RouteBasicAuthPlugin {
    pub fn new() -> Self {
        Self(PluginDefinition {
            name: "basic-auth".to_string(),
            entities: [PluginEntities::Route, PluginEntities::Service]
                .iter()
                .cloned()
                .collect(),
            options: vec![PluginOption {
                default_value: Some("false".to_string()),
                description:
                    "Set to true will not pass the authorization request headers to the Upstream."
                        .to_string(),
                is_required: Required::False,
                name: "hide_credentials".to_string(),
                property_type: PropertyType::Boolean,
            }],
        })
    }
}
