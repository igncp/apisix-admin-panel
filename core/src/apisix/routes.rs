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
pub struct RouteValue(pub EntityValue);}

pub type Route = Unit<RouteValue>;

impl Route {
    pub const API_PREFIX: &'static str = "/routes";
    pub const DISPLAY_LONG: &'static [&'static str] = &[];
    pub const DISPLAY_SHORT: &'static [&'static str] = &["uri"];
    pub const DOCS_KEY: &'static str = "route";
    pub const PLUGIN_ENTITY: Option<PluginEntities> = Some(PluginEntities::Route);
}

pub type GetRoutesResponse = GetListResponse<Route>;
pub type RouteEntity = Entity<Route>;

impl RouteEntity {
    pub fn create(&self) -> Result<ProxyFetchOpts, String> {
        let mut new_route_values = self.parsed.value.0.get_cloned();
        let existing_plugins = new_route_values
            .get("plugins")
            .unwrap_or(&json!({}))
            .clone();

        let existing_uri = new_route_values
            .get("uri")
            .map(|v| v.as_str().unwrap())
            .unwrap_or("");

        let parsed_uri = format!("/{}", existing_uri.trim_start_matches('/'));
        new_route_values.insert("uri".into(), json!(parsed_uri));

        new_route_values.insert(
            "plugins".into(),
            serde_json::to_value(existing_plugins).unwrap(),
        );

        let data = serde_json::to_string(&new_route_values).ok();

        Ok(ProxyFetchOpts {
            uri: Route::API_PREFIX.to_string(),
            method: ProxyFetchMethod::POST,
            data,
        })
    }

    pub fn delete(&self) -> Result<ProxyFetchOpts, String> {
        let id = self.parsed.value.0.get("id").unwrap().as_str().unwrap();
        let uri = format!("{}/{}", Route::API_PREFIX, id);
        ProxyFetchOpts::del(uri)
    }
}

impl RouteEntity {
    pub fn value_fields() -> Vec<EntityFields> {
        vec![
            EntityFields {
                hidden: false,
                default_value: None,
                description: "Matches with domain names such as foo.com or PAN domain names like *.foo.com.".to_string(),
                is_required: Required::False,
                name: "hosts".to_string(),
                property_type: PropertyType::List(Box::new(PropertyType::String)),
            },
            EntityFields {
                hidden: false,
                default_value: None,
                description: "Identifier for the Route.".to_string(),
                is_required: Required::False,
                name: "name".to_string(),
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
                description: "Matches with the specified methods. Matches all methods if empty or unspecified.".to_string(),
                is_required: Required::False,
                name: "methods".to_string(),
                property_type: PropertyType::List(Box::new(PropertyType::String)),
            },
            EntityFields {
                hidden: false,
                default_value: None,
                description: "Configuration of the Upstream.".to_string(),
                is_required: Required::False,
                name: "upstream".to_string(),
                property_type: PropertyType::Value,
            },
            EntityFields {
                hidden: false,
                default_value: None,
                description: "Matches the uri. For more advanced matching see Router.".to_string(),
                is_required: Required::TrueIfOtherMissing(vec!["uris".to_string()]),
                name: "uri".to_string(),
                property_type: PropertyType::String,
            },
            EntityFields {
                hidden: false,
                default_value: None,
                description: "".to_string(),
                is_required: Required::False,
                name: "plugins".to_string(),
                property_type: PropertyType::Plugins,
            },
        ]
    }
}
