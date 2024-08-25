use super::common::{PluginDefinition, PluginEntities, PluginOption, PluginPropertyType};
use crate::macros::derive_common_default;

derive_common_default! {
pub struct RouteLimitCountPlugin(PluginDefinition);}

impl RouteLimitCountPlugin {
    pub fn new() -> Self {
        Self(PluginDefinition {
            name: "limit-count".to_string(),
            entities: [PluginEntities::Route].iter().cloned().collect(),
            options: vec![
                PluginOption {
                    default_value: None,
                    description: "Maximum number of requests to allow.".to_string(),
                    is_required: true,
                    name: "count".to_string(),
                    property_type: PluginPropertyType::Number,
                },
                PluginOption {
                    default_value: None,
                    description: "Time in seconds before count is reset.".to_string(),
                    is_required: true,
                    name: "time_window".to_string(),
                    property_type: PluginPropertyType::Number,
                },
            ],
        })
    }
}
