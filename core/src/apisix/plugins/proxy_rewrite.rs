use super::common::{PluginDefinition, PluginEntities, PluginOption, PluginPropertyType};
use crate::macros::derive_common_default;

derive_common_default! {
pub struct ProxyRewritePlugin(PluginDefinition);}

impl ProxyRewritePlugin {
    pub fn new() -> Self {
        Self(PluginDefinition {
            name: "proxy-rewrite".to_string(),
            entities: [PluginEntities::Route, PluginEntities::Service].iter().cloned().collect(),
            options: vec![
                PluginOption {
                    description: "New Upstream forwarding address. Value supports Nginx variables. For example, $arg_name.".to_string(),
                    name: "uri".to_string(),
                    ..Default::default()
                },
                PluginOption {
                    description: "Rewrites the HTTP method.".to_string(),
                    name: "method".to_string(),
                    ..Default::default()
                },
                PluginOption {
                    name: "regex_uri".to_string(),
                    property_type: PluginPropertyType::List(Box::new(PluginPropertyType::String)),
                    ..Default::default()
                },
                PluginOption {
                    description: "New Upstream host address.".to_string(),
                    name: "host".to_string(),
                    ..Default::default()
                },
                PluginOption {
                    name: "headers".to_string(),
                    property_type: PluginPropertyType::Value,
                    ..Default::default()
                },
                PluginOption {
                    name: "use_real_request_uri_unsafe".to_string(),
                    description: "Use real_request_uri (original $request_uri in nginx) to bypass URI normalization. Enabling this is considered unsafe as it bypasses all URI normalization steps.".to_string(),
                    property_type: PluginPropertyType::Boolean,
                    ..Default::default()
                }
            ],
        })
    }
}
