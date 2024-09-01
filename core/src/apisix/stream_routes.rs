use super::{
    common::{prelude::*, Entity, EntityFields, EntityValue, GetListResponse, Unit},
    plugins::common::PluginEntities,
};
use crate::{apisix::base::Required, macros::derive_common_default, proxy::ProxyFetchOpts};

derive_common_default! {
pub struct StreamRouteValue(pub EntityValue);}

pub type StreamRoute = Unit<StreamRouteValue>;

impl EntityItemTrait for StreamRoute {
    const API_PREFIX: &'static str = "/stream_routes";
    const DOCS_KEY: &'static str = "stream-route";
    const PLUGIN_ENTITY: Option<PluginEntities> = None;
    const REQUIRED_VERSION: Option<&'static str> = Some("3.2.1");

    entity_trait_get_value!();
}

pub type StreamRouteEntity = Entity<StreamRoute>;

pub type GetStreamRoutesResponse = GetListResponse<StreamRoute>;

impl EntityTrait for StreamRouteEntity {
    fn create(&self) -> Result<ProxyFetchOpts, String> {
        let (id, data) = self.get_common_parsed_values();
        let (uri, method) = EntityValue::common_create(StreamRoute::API_PREFIX, id);

        Ok(ProxyFetchOpts { uri, method, data })
    }

    fn update(&self) -> Result<ProxyFetchOpts, String> {
        let (id, data) = self.get_common_parsed_values();
        let (uri, method) = EntityValue::common_update(StreamRoute::API_PREFIX, id);

        Ok(ProxyFetchOpts { uri, method, data })
    }

    fn value_fields() -> Vec<EntityFields> {
        vec![
            EntityFields {
                description: "Id of the Upstream service.".to_string(),
                name: "service_id".to_string(),
                ..Default::default()
            },
            EntityFields {
                description: "Id of the Service service.".to_string(),
                name: "upstream_id".to_string(),
                ..Default::default()
            },
            EntityFields {
                is_required: Required::True,
                name: "id".to_string(),
                is_editable: false,
                ..Default::default()
            },
        ]
    }
}
