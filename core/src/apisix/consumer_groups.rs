use serde_json::{json, Value};

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
pub struct ConsumerGroupValue(pub EntityValue);}

pub type ConsumerGroup = Unit<ConsumerGroupValue>;

impl ConsumerGroup {
    pub const API_PREFIX: &'static str = "/consumer_groups";
    pub const DISPLAY_LONG: &'static [&'static str] = &[];
    pub const DISPLAY_SHORT: &'static [&'static str] = &[];
    pub const DOCS_KEY: &'static str = "consumer-group";
    pub const PLUGIN_ENTITY: Option<PluginEntities> = Some(PluginEntities::ConsumerGroup);
}

pub type ConsumerGroupEntity = Entity<ConsumerGroup>;

pub type GetConsumerGroupsResponse = GetListResponse<ConsumerGroup>;

impl ConsumerGroupEntity {
    fn check_id(&self) -> Result<String, String> {
        let default_id = Value::String("".to_string());
        let values = self.parsed.value.0.get_cloned();
        let id = values.get("id").unwrap_or(&default_id).as_str().unwrap();

        if id.is_empty() {
            Err("id_required".to_string())
        } else {
            Ok(id.to_string())
        }
    }

    pub fn create(&self) -> Result<ProxyFetchOpts, String> {
        let id = self.check_id()?;

        let mut new_route_values = self.parsed.value.0.get_cloned();
        new_route_values.insert("plugins".into(), json!({}));
        new_route_values.insert("id".into(), json!(id));

        let data = serde_json::to_string(&new_route_values)
            .map_err(|e| e.to_string())?
            .into();

        Ok(ProxyFetchOpts {
            uri: ConsumerGroup::API_PREFIX.to_string(),
            method: ProxyFetchMethod::PUT,
            data,
        })
    }

    pub fn delete(&self) -> Result<ProxyFetchOpts, String> {
        let id = self.check_id()?;
        let uri = format!("{}/{}", ConsumerGroup::API_PREFIX, id);
        ProxyFetchOpts::del(uri)
    }
}

impl ConsumerGroupEntity {
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
                description: "".to_string(),
                is_required: Required::True,
                name: "id".to_string(),
                property_type: PropertyType::String,
            },
        ]
    }
}
