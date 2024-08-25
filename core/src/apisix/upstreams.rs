use super::{
    common::{Entity, EntityValue, GetListResponse, Unit},
    plugins::common::PluginEntities,
};
use crate::{
    apisix::common::{EntityFields, PropertyType, Required},
    macros::derive_common_default,
    proxy::{ProxyFetchMethod, ProxyFetchOpts},
};
use serde_json::json;

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
        let new_service = self.parsed.value.0.get_cloned();
        let id = self.parsed.value.0.get_str("id");

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

        Ok(ProxyFetchOpts {
            uri,
            method: ProxyFetchMethod::POST,
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
                hidden: true,
                default_value: None,
                description: "".to_string(),
                is_required: Required::False,
                name: "id".to_string(),
                property_type: PropertyType::String,
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
                description: "Identifier for the Upstream.".to_string(),
                is_required: Required::False,
                name: "name".to_string(),
                property_type: PropertyType::String,
            },
        ]
    }
}
