use super::{
    common::{prelude::*, Entity, EntityValue, GetListResponse, Unit},
    plugins::common::PluginEntities,
};
use crate::{
    apisix::{base::Required, common::EntityFields},
    macros::derive_common_default,
    proxy::{ProxyFetchMethod, ProxyFetchOpts},
};

derive_common_default! {
pub struct SecretValue(pub EntityValue);}

pub type Secret = Unit<SecretValue>;

impl EntityItemTrait for Secret {
    const API_PREFIX: &'static str = "/secrets/vault";
    const DOCS_KEY: &'static str = "secret";
    const PLUGIN_ENTITY: Option<PluginEntities> = None;

    entity_trait_get_value!();
}

pub type GetSecretsResponse = GetListResponse<Secret>;
pub type SecretEntity = Entity<Secret>;

impl EntityTrait for SecretEntity {
    fn create(&self) -> Result<ProxyFetchOpts, String> {
        let (_, data) = self.get_common_parsed_values();
        let uri = format!("{}/vault", Secret::API_PREFIX).to_string();

        Ok(ProxyFetchOpts {
            uri,
            method: ProxyFetchMethod::PUT,
            data,
        })
    }

    fn update(&self) -> Result<ProxyFetchOpts, String> {
        let (id, data) = self.get_common_parsed_values();
        let uri = format!("{}/vault/{}", Secret::API_PREFIX, id).to_string();

        Ok(ProxyFetchOpts {
            uri,
            method: ProxyFetchMethod::PATCH,
            data,
        })
    }

    fn value_fields() -> Vec<EntityFields> {
        vec![
            EntityFields {
                description:
                    "Matches with domain names such as foo.com or PAN domain names like *.foo.com."
                        .to_string(),
                is_required: Required::True,
                name: "uri".to_string(),
                ..EntityFields::default()
            },
            EntityFields {
                description: "key prefix".to_string(),
                is_required: Required::True,
                name: "prefix".to_string(),
                ..EntityFields::default()
            },
            EntityFields {
                description: "vault token".to_string(),
                is_required: Required::True,
                name: "token".to_string(),
                ..EntityFields::default()
            },
            EntityFields {
                description: "Vault namespace, no default value".to_string(),
                name: "namespace".to_string(),
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
