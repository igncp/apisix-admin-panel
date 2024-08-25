use super::{
    common::{Entity, EntityValue, GetListResponse, Unit},
    plugins::common::PluginEntities,
};
use crate::{
    apisix::common::{EntityFields, PropertyType, Required},
    macros::derive_common_default,
    proxy::{ProxyFetchMethod, ProxyFetchOpts},
};

derive_common_default! {
pub struct SecretValue(pub EntityValue);}

pub type Secret = Unit<SecretValue>;

impl Secret {
    pub const API_PREFIX: &'static str = "/secrets";
    pub const DISPLAY_LONG: &'static [&'static str] = &[];
    pub const DISPLAY_SHORT: &'static [&'static str] = &[];
    pub const DOCS_KEY: &'static str = "secret";
    pub const PLUGIN_ENTITY: Option<PluginEntities> = None;
}

pub type GetSecretsResponse = GetListResponse<Secret>;
pub type SecretEntity = Entity<Secret>;

impl SecretEntity {
    pub fn create(&self) -> Result<ProxyFetchOpts, String> {
        let new_route_values = self.parsed.value.0.get_cloned();

        let data = serde_json::to_string(&new_route_values).ok();
        let uri = format!("{}/vault", Secret::API_PREFIX).to_string();

        Ok(ProxyFetchOpts {
            uri,
            method: ProxyFetchMethod::PUT,
            data,
        })
    }

    pub fn delete(&self) -> Result<ProxyFetchOpts, String> {
        let id = self.parsed.value.0.get("id").unwrap().as_str().unwrap();
        let uri = format!("{}/vault/{}", Secret::API_PREFIX, id);
        ProxyFetchOpts::del(uri)
    }
}

impl SecretEntity {
    pub fn value_fields() -> Vec<EntityFields> {
        vec![
            EntityFields {
                hidden: false,
                default_value: None,
                description:
                    "Matches with domain names such as foo.com or PAN domain names like *.foo.com."
                        .to_string(),
                is_required: Required::True,
                name: "uri".to_string(),
                property_type: PropertyType::String,
            },
            EntityFields {
                hidden: false,
                default_value: None,
                description: "key prefix".to_string(),
                is_required: Required::True,
                name: "prefix".to_string(),
                property_type: PropertyType::String,
            },
            EntityFields {
                hidden: false,
                default_value: None,
                description: "vault token".to_string(),
                is_required: Required::True,
                name: "token".to_string(),
                property_type: PropertyType::String,
            },
            EntityFields {
                hidden: false,
                default_value: None,
                description: "Vault namespace, no default value".to_string(),
                is_required: Required::False,
                name: "namespace".to_string(),
                property_type: PropertyType::String,
            },
            EntityFields {
                hidden: true,
                default_value: None,
                description: "".to_string(),
                is_required: Required::True,
                name: "id".to_string(),
                property_type: PropertyType::String,
            },
        ]
    }
}
