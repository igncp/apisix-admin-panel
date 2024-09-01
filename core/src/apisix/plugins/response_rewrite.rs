use super::common::{PluginDefinition, PluginEntities, PluginOption};
use crate::{apisix::base::PropertyType, macros::derive_common_default};

derive_common_default! {
pub struct ResponseRewritePlugin(PluginDefinition);}

impl ResponseRewritePlugin {
    pub fn new() -> Self {
        Self(PluginDefinition {
            name: "response-rewrite".to_string(),
            entities: [PluginEntities::Route, PluginEntities::Service].iter().cloned().collect(),
            options: vec![
                PluginOption {
                    description: "New HTTP status code in the response. If unset, falls back to the original status code.".to_string(),
                    name: "status_code".to_string(),
                    property_type: PropertyType::Number,
                    ..Default::default()
                },
                PluginOption {
                    description: "New body of the response. The content-length would also be reset.".to_string(),
                    name: "body".to_string(),
                    ..Default::default()
                },
                PluginOption {
                    description: "When set, the body passed in body will be decoded before writing to the client which is used in some image and Protobuffer scenarios. Note that this field only allows decoding the body passed in plugin configuration and does not decode upstream response.".to_string(),
                    name: "body_base64".to_string(),
                    property_type: PropertyType::Boolean,
                    ..Default::default()
                },
                PluginOption {
                    name: "headers".to_string(),
                    property_type: PropertyType::JSON,
                    ..Default::default()
                },
                PluginOption {
                    description: "Nginx variable expressions to conditionally execute the rewrite. The Plugin will be executed unconditionally if this value is empty.".to_string(),
                    name: "vars".to_string(),
                    property_type: PropertyType::List(
                        Box::new(PropertyType::JSON),
                    ),
                    ..Default::default()
                },
                PluginOption {
                    description: "List of filters that modify the response body by replacing one specified string with another.".to_string(),
                    name: "filters".to_string(),
                    property_type: PropertyType::List(
                        Box::new(PropertyType::JSON),
                    ),
                    ..Default::default()
                }
            ],
        })
    }
}
