use super::common::{PluginDefinition, PluginEntities, PluginOption, PluginPropertyType};
use crate::macros::derive_common_default;

derive_common_default! {
pub struct RouteKeyAuthPlugin(PluginDefinition);}

impl RouteKeyAuthPlugin {
    pub fn new() -> Self {
        Self(PluginDefinition {
            name: "key-auth".to_string(),
            entities: [PluginEntities::Route, PluginEntities::Service]
                .iter()
                .cloned()
                .collect(),
            options: vec![
                PluginOption {
                    default_value: Some("apikey".to_string()),
                    description: "The header to get the key from.".to_string(),
                    is_required: false,
                    name: "header".to_string(),
                    property_type: PluginPropertyType::String,
                },
                PluginOption {
                    default_value: Some("apikey".to_string()),
                    description:
                        "The query string to get the key from. Lower priority than header.".to_string(),
                    is_required: false,
                    name: "query".to_string(),
                    property_type: PluginPropertyType::String,
                },
                PluginOption {
                    default_value: Some("false".to_string()),
                    description: "Apache APISIX will pass the request header or query string that contains the authentication information to the Upstream if hide_credentials is false. Otherwise the authentication information will be removed before proxying.".to_string(),
                    is_required: false,
                    name: "hide_credentials".to_string(),
                    property_type: PluginPropertyType::Boolean,
                },
            ],
        })
    }
}

derive_common_default! {
pub struct ConsumerKeyAuthPlugin(PluginDefinition);}

impl ConsumerKeyAuthPlugin {
    pub fn new() -> Self {
        Self(PluginDefinition {
            name: "key-auth".to_string(),
            entities: [PluginEntities::Consumer]
                .iter()
                .cloned()
                .collect(),
            options: vec![
                PluginOption {
                    default_value: None,
                    description: "Unique key for a Consumer. This field supports saving the value in Secret Manager using the APISIX Secret resource.".to_string(),
                    is_required: true,
                    name: "key".to_string(),
                    property_type: PluginPropertyType::String,
                },
            ],
        })
    }
}
