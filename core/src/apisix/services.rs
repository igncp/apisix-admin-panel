use super::{
    common::{Entity, EntityFields, EntityValue, GetListResponse, Unit},
    plugins::common::PluginEntities,
};
use crate::{
    apisix::common::PropertyType,
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
        let mut values = self.parsed.value.0.get_cloned();
        let id = self.parsed.value.0.get_str("id");
        values.remove("id");
        let uri = format!(
            "{}{}",
            Service::API_PREFIX,
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
        let data = serde_json::to_string(&values).ok();

        Ok(ProxyFetchOpts { uri, method, data })
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
                description: "Unique text within the services".to_string(),
                name: "id".to_string(),
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
                property_type: PropertyType::Value,
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
                property_type: PropertyType::Value,
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
