use super::{
    common::{prelude::*, Entity, EntityFields, EntityValue, GetListResponse, Unit},
    plugins::common::PluginEntities,
};
use crate::{apisix::base::PropertyType, macros::derive_common_default, proxy::ProxyFetchOpts};

derive_common_default! {
pub struct ServiceValue(pub EntityValue);}

pub type Service = Unit<ServiceValue>;

impl EntityItemTrait for Service {
    const API_PREFIX: &'static str = "/services";
    const DISPLAY_LONG: &'static [&'static str] = &[];
    const DISPLAY_SHORT: &'static [&'static str] = &[];
    const DOCS_KEY: &'static str = "service";
    const PLUGIN_ENTITY: Option<PluginEntities> = Some(PluginEntities::Service);

    entity_trait_get_value!();
}

pub type ServiceEntity = Entity<Service>;

pub type GetServicesResponse = GetListResponse<Service>;

impl EntityTrait for ServiceEntity {
    fn create(&self) -> Result<ProxyFetchOpts, String> {
        let (id, data) = self.get_common_parsed_values();
        let (uri, method) = EntityValue::common_create(Service::API_PREFIX, id);

        Ok(ProxyFetchOpts { uri, method, data })
    }

    fn update(&self) -> Result<ProxyFetchOpts, String> {
        let (id, data) = self.get_common_parsed_values();
        let (uri, method) = EntityValue::common_update(Service::API_PREFIX, id);

        Ok(ProxyFetchOpts { uri, method, data })
    }

    fn value_fields() -> Vec<EntityFields> {
        vec![
            EntityFields {
                description: "Unique text within the services".to_string(),
                name: "id".to_string(),
                is_editable: false,
                ..EntityFields::default()
            },
            EntityFields {
                description: "Identifier for the Service.".to_string(),
                name: "name".to_string(),
                ..EntityFields::default()
            },
            EntityFields {
                description: "Plugins that are executed during the request/response cycle.".to_string(),
                name: "plugins".to_string(),
                property_type: PropertyType::Plugins,
                ..EntityFields::default()
            },
            EntityFields {
                description: "Configuration of the Upstream.".to_string(),
                name: "upstream".to_string(),
                property_type: PropertyType::JSON,
                ..EntityFields::default()
            },
            EntityFields {
                description: "Id of the Upstream service.".to_string(),
                name: "upstream_id".to_string(),
                ..EntityFields::default()
            },
            EntityFields {
                description: "Description of usage scenarios.".to_string(),
                name: "desc".to_string(),
                ..EntityFields::default()
            },
            EntityFields {
                description: "Attributes of the Service specified as key-value pairs.".to_string(),
                name: "labels".to_string(),
                property_type: PropertyType::JSON,
                ..EntityFields::default()
            },
            EntityFields {
                description: "Matches with any one of the multiple hosts specified in the form of a non-empty list.".to_string(),
                name: "hosts".to_string(),
                property_type: PropertyType::List(
                    Box::new(PropertyType::String)
                ),
                ..EntityFields::default()
            },
            EntityFields {
                description: "Enables a websocket. Set to false by default.".to_string(),
                name: "enable_websocket".to_string(),
                property_type: PropertyType::Boolean,
                ..EntityFields::default()
            },
        ]
    }
}
