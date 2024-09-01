use super::{
    common::{prelude::*, Entity, EntityFields, EntityValue, GetListResponse, Unit},
    plugins::common::PluginEntities,
};
use crate::{
    apisix::base::{PropertyType, Required},
    macros::derive_common_default,
    proxy::ProxyFetchOpts,
};

derive_common_default! {
pub struct ConsumerGroupValue(pub EntityValue);}

pub type ConsumerGroup = Unit<ConsumerGroupValue>;

impl EntityItemTrait for ConsumerGroup {
    const API_PREFIX: &'static str = "/consumer_groups";
    const DISPLAY_LONG: &'static [&'static str] = &[];
    const DISPLAY_SHORT: &'static [&'static str] = &[];
    const DOCS_KEY: &'static str = "consumer-group";
    const PLUGIN_ENTITY: Option<PluginEntities> = Some(PluginEntities::ConsumerGroup);

    entity_trait_get_value!();
}

pub type ConsumerGroupEntity = Entity<ConsumerGroup>;

pub type GetConsumerGroupsResponse = GetListResponse<ConsumerGroup>;

impl EntityTrait for ConsumerGroupEntity {
    fn create(&self) -> Result<ProxyFetchOpts, String> {
        let id = self.check_id()?;

        let new_route_values = self.parsed.value.0.get_cloned();

        let (uri, method) = EntityValue::common_create(ConsumerGroup::API_PREFIX, id);

        let data = serde_json::to_string(&new_route_values)
            .map_err(|e| e.to_string())?
            .into();

        Ok(ProxyFetchOpts { uri, method, data })
    }

    fn update(&self) -> Result<ProxyFetchOpts, String> {
        let id = self.check_id()?;

        let new_route_values = self.parsed.value.0.get_cloned();

        let (uri, method) = EntityValue::common_create(ConsumerGroup::API_PREFIX, id);

        let data = serde_json::to_string(&new_route_values)
            .map_err(|e| e.to_string())?
            .into();

        Ok(ProxyFetchOpts { uri, method, data })
    }

    fn value_fields() -> Vec<EntityFields> {
        vec![
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
                is_required: Required::True,
                name: "id".to_string(),
                is_editable: false,
                ..EntityFields::default()
            },
        ]
    }
}
