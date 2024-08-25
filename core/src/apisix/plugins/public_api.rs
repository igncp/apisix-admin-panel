use super::common::{PluginDefinition, PluginEntities, PluginOption};
use crate::macros::derive_common_default;

derive_common_default! {
pub struct PublicApiPlugin(PluginDefinition);}

impl PublicApiPlugin {
    pub fn new() -> Self {
        Self(PluginDefinition {
            name: "public-api".to_string(),
            entities: [PluginEntities::Route].iter().cloned().collect(),
            options: vec![PluginOption {
                description: "URI of the public API. When setting up a Route, use this attribute to configure the original public API URI.".to_string(),
                name: "uri".to_string(),
                ..Default::default()
            }],
        })
    }
}
