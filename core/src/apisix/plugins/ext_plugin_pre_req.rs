use super::common::{PluginDefinition, PluginEntities, PluginOption};
use crate::{apisix::base::PropertyType, macros::derive_common_default};

derive_common_default! {
pub struct ExtPluginPreReqPlugin(PluginDefinition);}

impl ExtPluginPreReqPlugin {
    pub fn new() -> Self {
        Self(PluginDefinition {
            name: "ext-plugin-pre-req".to_string(),
            entities: [
                PluginEntities::Route,
                PluginEntities::Consumer,
                PluginEntities::ConsumerGroup,
                PluginEntities::Service,
            ]
            .iter()
            .cloned()
            .collect(),
            options: vec![
                PluginOption {
                    name: "conf".to_string(),
                    property_type: PropertyType::List(
                        Box::new(PropertyType::JSON)
                    ),
                    description: "List of Plugins and their configurations to be executed on the Plugin Runner.".to_string(),
                    ..Default::default()
                },
                PluginOption {
                    name: "allow_degradation".to_string(),
                    property_type: PropertyType::Boolean,
                    description: "Sets Plugin degradation when the Plugin Runner is not available. When set to true, requests are allowed to continue.".to_string(),
                    ..Default::default()
                }
            ],
        })
    }
}
