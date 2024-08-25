use super::{
    common::{Entity, EntityValue, GetListResponse, Unit},
    plugins::common::PluginEntities,
};
use crate::{
    apisix::common::{EntityFields, PropertyType},
    macros::derive_common_default,
    proxy::{ProxyFetchMethod, ProxyFetchOpts},
};

derive_common_default! {
pub struct UpstreamValue(pub EntityValue);}

pub type Upstream = Unit<UpstreamValue>;

impl Upstream {
    pub const API_PREFIX: &'static str = "/upstreams";
    pub const DISPLAY_LONG: &'static [&'static str] = &[];
    pub const DISPLAY_SHORT: &'static [&'static str] = &[];
    pub const DOCS_KEY: &'static str = "upstream";
    pub const PLUGIN_ENTITY: Option<PluginEntities> = None;
}

pub type UpstreamEntity = Entity<Upstream>;

pub type GetUpstreamsResponse = GetListResponse<Upstream>;

impl UpstreamEntity {
    pub fn create(&self) -> Result<ProxyFetchOpts, String> {
        let id = self.parsed.value.0.get_str("id");
        let mut new_service = self.parsed.value.0.get_cloned();
        new_service.remove("id");

        let opts = serde_json::to_string(&new_service).unwrap();
        let uri = format!(
            "{}{}",
            Upstream::API_PREFIX,
            if id.is_empty() {
                "".to_string()
            } else {
                format!("/{}", id)
            }
        );
        let method = if id.is_empty() {
            ProxyFetchMethod::POST
        } else {
            ProxyFetchMethod::PUT
        };

        Ok(ProxyFetchOpts {
            uri,
            method,
            data: Some(opts),
        })
    }

    pub fn delete(&self) -> Result<ProxyFetchOpts, String> {
        let id = self.parsed.value.0.get_str("id");
        let uri = format!("{}/{}", Upstream::API_PREFIX, id);
        ProxyFetchOpts::del(uri)
    }
}

impl UpstreamEntity {
    pub fn value_fields() -> Vec<EntityFields> {
        vec![
            EntityFields {
                description: "Unique text within the upstreams".to_string(),
                name: "id".to_string(),
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
                    r#"{"web_server_1:80": 1}"#.to_string()
                ),
                property_type: PropertyType::Value,
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
                property_type: PropertyType::Value,
                ..Default::default()
            },
        ]
    }
}
