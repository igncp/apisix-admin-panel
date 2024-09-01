use super::{
    common::{prelude::*, Entity, EntityValue, GetListResponse, Unit},
    plugins::common::PluginEntities,
};
use crate::{
    apisix::{base::PropertyType, common::EntityFields},
    macros::derive_common_default,
    proxy::ProxyFetchOpts,
};

derive_common_default! {
pub struct UpstreamValue(pub EntityValue);}

pub type Upstream = Unit<UpstreamValue>;

impl EntityItemTrait for Upstream {
    const API_PREFIX: &'static str = "/upstreams";
    const DOCS_KEY: &'static str = "upstream";
    const PLUGIN_ENTITY: Option<PluginEntities> = None;

    entity_trait_get_value!();
}

pub type UpstreamEntity = Entity<Upstream>;

pub type GetUpstreamsResponse = GetListResponse<Upstream>;

impl EntityTrait for UpstreamEntity {
    fn create(&self) -> Result<ProxyFetchOpts, String> {
        let (id, data) = self.get_common_parsed_values();
        let (uri, method) = EntityValue::common_create(Upstream::API_PREFIX, id);

        Ok(ProxyFetchOpts { uri, method, data })
    }

    fn update(&self) -> Result<ProxyFetchOpts, String> {
        let (id, data) = self.get_common_parsed_values();
        let (uri, method) = EntityValue::common_update(Upstream::API_PREFIX, id);

        Ok(ProxyFetchOpts { uri, method, data })
    }

    fn value_fields() -> Vec<EntityFields> {
        vec![
            EntityFields {
                description: "Unique text within the upstreams".to_string(),
                name: "id".to_string(),
                is_editable: false,
                ..Default::default()
            },
            EntityFields {
                description: "Identifier for the Upstream.".to_string(),
                name: "name".to_string(),
                ..Default::default()
            },
            EntityFields {
                description: "Description of usage scenarios.".to_string(),
                name: "desc".to_string(),
                ..Default::default()
            },
            EntityFields {
                description:
                    "Load balancing algorithm to be used, and the default value is roundrobin."
                        .to_string(),
                name: "type".to_string(),
                ..Default::default()
            },
            EntityFields {
                name: "nodes".to_string(),
                example: Some(
                    r#"{"web-server-1:80": 1}"#.to_string()
                ),
                property_type: PropertyType::JSON,
                ..Default::default()
            },
            EntityFields {
                description: "Service name used for service discovery.".to_string(),
                name: "service_name".to_string(),
                ..Default::default()
            },
            EntityFields {
                description: "The type of service discovery.".to_string(),
                name: "discovery_type".to_string(),
                ..Default::default()
            },
            EntityFields {
                description: "Timeout to continue with retries. Setting this to 0 disables the retry timeout.".to_string(),
                name: "retry_timeout".to_string(),
                property_type: PropertyType::Number,
                ..Default::default()
            },
            EntityFields {
                description: "Sets the timeout (in seconds) for connecting to, and sending and receiving messages to and from the Upstream.".to_string(),
                example: Some(r#"{"connect": 0.5,"send": 0.5,"read": 0.5}"#.to_string()),
                name: "timeout".to_string(),
                property_type: PropertyType::JSON,
                ..Default::default()
            },
            EntityFields {
                description: "The scheme used when communicating with the Upstream.".to_string(),
                name: "scheme".to_string(),
                property_type: PropertyType::Enum(vec![
                    "http".to_string(),
                    "https".to_string(),
                    "grpc".to_string(),
                    "grpcs".to_string(),
                ]),
                ..Default::default()
            },
            EntityFields {
                description: "Attributes of the Upstream specified as key-value pairs.".to_string(),
                name: "labels".to_string(),
                property_type: PropertyType::JSON,
                ..Default::default()
            },
        ]
    }
}
