use super::{
    common::{Entity, EntityFields, EntityValue, GetListResponse, Unit},
    plugins::common::PluginEntities,
};
use crate::{
    apisix::common::{PropertyType, Required},
    macros::derive_common_default,
    proxy::{ProxyFetchMethod, ProxyFetchOpts},
};

derive_common_default! {
pub struct ConsumerValue(pub EntityValue);}

pub type Consumer = Unit<ConsumerValue>;

impl Consumer {
    pub const API_PREFIX: &'static str = "/consumers";
    pub const DISPLAY_LONG: &'static [&'static str] = &[];
    pub const DISPLAY_SHORT: &'static [&'static str] = &["username"];
    pub const DOCS_KEY: &'static str = "consumer";
    pub const PLUGIN_ENTITY: Option<PluginEntities> = Some(PluginEntities::Consumer);
}

pub type ConsumerEntity = Entity<Consumer>;

pub type GetConsumersResponse = GetListResponse<Consumer>;

impl ConsumerEntity {
    pub fn create(&self) -> Result<ProxyFetchOpts, String> {
        let new_route_values = self.parsed.value.0.get_cloned();
        let opts = serde_json::to_string(&new_route_values).unwrap();

        Ok(ProxyFetchOpts {
            uri: Consumer::API_PREFIX.to_string(),
            method: ProxyFetchMethod::PUT,
            data: Some(opts),
        })
    }

    pub fn delete(&self) -> Result<ProxyFetchOpts, String> {
        let uri = format!(
            "{}/{}",
            Consumer::API_PREFIX,
            self.parsed.value.0.get_str("username")
        );
        ProxyFetchOpts::del(uri)
    }
}

impl ConsumerEntity {
    pub fn value_fields() -> Vec<EntityFields> {
        vec![
            EntityFields {
                hidden: false,
                default_value: None,
                description: "".to_string(),
                is_required: Required::False,
                name: "plugins".to_string(),
                property_type: PropertyType::Plugins,
            },
            EntityFields {
                hidden: false,
                default_value: None,
                description: "Description of usage scenarios.".to_string(),
                is_required: Required::False,
                name: "desc".to_string(),
                property_type: PropertyType::String,
            },
            EntityFields {
                hidden: false,
                default_value: None,
                description: "Name of the Consumer.".to_string(),
                is_required: Required::True,
                name: "username".to_string(),
                property_type: PropertyType::String,
            },
            EntityFields {
                hidden: false,
                default_value: None,
                description: "Group of the Consumer.".to_string(),
                is_required: Required::False,
                name: "group_id".to_string(),
                property_type: PropertyType::String,
            },
        ]
    }
}
