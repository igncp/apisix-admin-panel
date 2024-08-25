use super::common::{PluginDefinition, PluginEntities, PluginOption, PluginPropertyType};
use crate::macros::derive_common_default;

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
                    is_required: true,
                    name: "username".to_string(),
                    property_type: PluginPropertyType::String,
                },
                PluginOption {
                    default_value: None,
                    description: "Password of the user. This field supports saving the value in Secret Manager using the APISIX Secret resource.".to_string(),
                    is_required: true,
                    name: "password".to_string(),
                    property_type: PluginPropertyType::String,
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
                is_required: false,
                name: "hide_credentials".to_string(),
                property_type: PluginPropertyType::Boolean,
            }],
        })
    }
}