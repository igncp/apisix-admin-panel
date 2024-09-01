use super::common::{PluginDefinition, PluginEntities, PluginOption};
use crate::{apisix::base::PropertyType, macros::derive_common_default};

derive_common_default! {
pub struct PrometheusPlugin(PluginDefinition);}

impl PrometheusPlugin {
    pub fn new() -> Self {
        Self(PluginDefinition {
            name: "prometheus".to_string(),
            entities: [PluginEntities::Route, PluginEntities::Service]
                .iter()
                .cloned()
                .collect(),
                options: vec![
                    PluginOption {
                        description: "When set to true, prints Route/Service name instead of ID in Prometheus metric.".to_string(),
                        name: "prefer_name".to_string(),
                        property_type: PropertyType::Boolean,
                        ..Default::default()
                    }
                ],
        })
    }
}
