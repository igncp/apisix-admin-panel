use super::common::{PluginDefinition, PluginEntities, PluginOption};
use crate::{apisix::base::PropertyType, macros::derive_common_default};

derive_common_default! {
pub struct IpRestrictionPlugin(PluginDefinition);}

impl IpRestrictionPlugin {
    pub fn new() -> Self {
        Self(PluginDefinition {
            name: "ip-restriction".to_string(),
            entities: [
                PluginEntities::Route,
                PluginEntities::Service,
                PluginEntities::Consumer,
            ]
            .iter()
            .cloned()
            .collect(),
            options: vec![
                PluginOption {
                    description: "List of IPs or CIDR ranges to whitelist.".to_string(),
                    name: "whitelist".to_string(),
                    property_type: PropertyType::List(Box::new(PropertyType::String)),
                    ..Default::default()
                },
                PluginOption {
                    description: "List of IPs or CIDR ranges to blacklist.".to_string(),
                    name: "blacklist".to_string(),
                    property_type: PropertyType::List(Box::new(PropertyType::String)),
                    ..Default::default()
                },
                PluginOption {
                    description: "Message returned when the IP address is not allowed access."
                        .to_string(),
                    name: "message".to_string(),
                    ..Default::default()
                },
            ],
        })
    }
}
