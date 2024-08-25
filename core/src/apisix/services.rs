use serde_json::Value;

use super::{
    common::{Entity, EntityFields, EntityValue, GetListResponse, Unit},
    plugins::common::PluginEntities,
};
use crate::{
    apisix::common::{PropertyType, Required},
    macros::derive_common_default,
    proxy::{ProxyFetchMethod, ProxyFetchOpts},
};

derive_common_default! {
pub struct ServiceValue(pub EntityValue);}

pub type Service = Unit<ServiceValue>;

impl Service {
    pub const API_PREFIX: &'static str = "/services";
    pub const DISPLAY_LONG: &'static [&'static str] = &[];
    pub const DISPLAY_SHORT: &'static [&'static str] = &[];
    pub const DOCS_KEY: &'static str = "service";
    pub const PLUGIN_ENTITY: Option<PluginEntities> = Some(PluginEntities::Service);
}

pub type ServiceEntity = Entity<Service>;

pub type GetServicesResponse = GetListResponse<Service>;

impl ServiceEntity {
    pub fn create(&self) -> Result<ProxyFetchOpts, String> {
        let values = self.parsed.value.0.get_cloned();
        let default_id = Value::String("".to_string());
        let id = values.get("id").unwrap_or(&default_id).as_str().unwrap();
        let uri = format!(
            "{}{}",
            Service::API_PREFIX,
            if id.is_empty() {
                "".to_string()
            } else {
                format!("/{}", id)
            }
        );
        let data = serde_json::to_string(&values).ok();

        Ok(ProxyFetchOpts {
            uri,
            method: ProxyFetchMethod::POST,
            data,
        })
    }

    pub fn delete(&self) -> Result<ProxyFetchOpts, String> {
        let id = self.parsed.value.0.get("id").unwrap().as_str().unwrap();
        let uri = format!("{}/{}", Service::API_PREFIX, id);
        ProxyFetchOpts::del(uri)
    }
}

impl ServiceEntity {
    pub fn value_fields() -> Vec<EntityFields> {
        vec![
            EntityFields {
                hidden: false,
                default_value: None,
                description: "Plugins that are executed during the request/response cycle.".to_string(),
                is_required: Required::False,
                name: "plugins".to_string(),
                property_type: PropertyType::Plugins
            },
            EntityFields {
                hidden: false,
                default_value: None,
                description: "	Configuration of the Upstream.".to_string(),
                is_required: Required::False,
                name: "upstream".to_string(),
                property_type: PropertyType::Value
            },
            EntityFields {
                hidden: false,
                default_value: None,
                description: "Id of the Upstream service.".to_string(),
                is_required: Required::False,
                name: "upstream_id".to_string(),
                property_type: PropertyType::String
            },
            EntityFields {
                hidden: false,
                default_value: None,
                description: "Identifier for the Service.".to_string(),
                is_required: Required::False,
                name: "name".to_string(),
                property_type: PropertyType::String
            },
            EntityFields {
                hidden: false,
                default_value: None,
                description: "Description of usage scenarios.".to_string(),
                is_required: Required::False,
                name: "desc".to_string(),
                property_type: PropertyType::String
            },
            EntityFields {
                hidden: false,
                default_value: None,
                description: "Attributes of the Service specified as key-value pairs.".to_string(),
                is_required: Required::False,
                name: "labels".to_string(),
                property_type: PropertyType::Value
            },
            EntityFields {
                hidden: false,
                default_value: None,
                description: "Matches with any one of the multiple hosts specified in the form of a non-empty list.".to_string(),
                is_required: Required::False,
                name: "hosts".to_string(),
                property_type: PropertyType::List(
                    Box::new(PropertyType::String)
                )
            },
            EntityFields {
                hidden: false,
                default_value: None,
                description: "Enables a websocket. Set to false by default.".to_string(),
                is_required: Required::False,
                name: "enable_websocket".to_string(),
                property_type: PropertyType::Boolean,
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
