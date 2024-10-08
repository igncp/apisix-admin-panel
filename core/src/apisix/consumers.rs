use super::{
    common::{prelude::*, Entity, EntityFields, EntityValue, GetListResponse, Unit},
    plugins::common::PluginEntities,
};
use crate::{
    apisix::base::{PropertyType, Required},
    macros::derive_common_default,
    proxy::{ProxyFetchMethod, ProxyFetchOpts},
};

derive_common_default! {
pub struct ConsumerValue(pub EntityValue);}

pub type Consumer = Unit<ConsumerValue>;

pub type ConsumerEntity = Entity<Consumer>;

impl EntityItemTrait for Consumer {
    const API_PREFIX: &'static str = "/consumers";
    const DISPLAY_SHORT: &'static [&'static str] = &["username"];
    const DOCS_KEY: &'static str = "consumer";
    const ID_NAME: &'static str = "username";
    const PLUGIN_ENTITY: Option<PluginEntities> = Some(PluginEntities::Consumer);

    entity_trait_get_value!();
}

pub type GetConsumersResponse = GetListResponse<Consumer>;

impl EntityTrait for ConsumerEntity {
    fn create(&self) -> Result<ProxyFetchOpts, String> {
        let new_route_values = self.parsed.get_cloned();
        let opts = serde_json::to_string(&new_route_values).unwrap();

        Ok(ProxyFetchOpts {
            uri: Consumer::API_PREFIX.to_string(),
            method: ProxyFetchMethod::PUT,
            data: Some(opts),
        })
    }

    fn update(&self) -> Result<ProxyFetchOpts, String> {
        let new_route_values = self.parsed.get_cloned();
        let opts = serde_json::to_string(&new_route_values).unwrap();

        Ok(ProxyFetchOpts {
            uri: Consumer::API_PREFIX.to_string(),
            method: ProxyFetchMethod::PUT,
            data: Some(opts),
        })
    }

    fn value_fields() -> Vec<EntityFields> {
        vec![
            EntityFields {
                description: "Name of the Consumer.".to_string(),
                is_required: Required::True,
                name: "username".to_string(),
                is_editable: false,
                ..EntityFields::default()
            },
            EntityFields {
                name: "plugins".to_string(),
                property_type: PropertyType::Plugins,
                ..EntityFields::default()
            },
            EntityFields {
                description: "Description of usage scenarios.".to_string(),
                name: "desc".to_string(),
                ..EntityFields::default()
            },
            EntityFields {
                description: "Group of the Consumer.".to_string(),
                name: "group_id".to_string(),
                ..EntityFields::default()
            },
        ]
    }
}
