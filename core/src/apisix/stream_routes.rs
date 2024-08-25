use super::{
    common::{Entity, EntityFields, EntityValue, GetListResponse, Unit},
    plugins::common::PluginEntities,
};
use crate::{
    apisix::common::Required,
    macros::derive_common_default,
    proxy::{ProxyFetchMethod, ProxyFetchOpts},
};

derive_common_default! {
pub struct StreamRouteValue(pub EntityValue);}

pub type StreamRoute = Unit<StreamRouteValue>;

impl StreamRoute {
    pub const API_PREFIX: &'static str = "/stream_routes";
    pub const DISPLAY_LONG: &'static [&'static str] = &[];
    pub const DISPLAY_SHORT: &'static [&'static str] = &[];
    pub const DOCS_KEY: &'static str = "stream-route";
    pub const PLUGIN_ENTITY: Option<PluginEntities> = None;
    pub const REQUIRED_VERSION: &'static str = "3.2.1";
}

pub type StreamRouteEntity = Entity<StreamRoute>;

pub type GetStreamRoutesResponse = GetListResponse<StreamRoute>;

impl StreamRouteEntity {
    pub fn create(&self) -> Result<ProxyFetchOpts, String> {
        let new_route_values = self.parsed.value.0.get_cloned();
        let opts = serde_json::to_string(&new_route_values).unwrap();

        Ok(ProxyFetchOpts {
            uri: StreamRoute::API_PREFIX.to_string(),
            method: ProxyFetchMethod::POST,
            data: Some(opts),
        })
    }

    pub fn delete(&self) -> Result<ProxyFetchOpts, String> {
        let uri = format!(
            "{}/{}",
            StreamRoute::API_PREFIX,
            self.parsed.value.0.get_str("id")
        );
        ProxyFetchOpts::del(uri)
    }
}

impl StreamRouteEntity {
    pub fn value_fields() -> Vec<EntityFields> {
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
                hidden: true,
                is_required: Required::True,
                name: "id".to_string(),
                ..Default::default()
            },
        ]
    }
}
