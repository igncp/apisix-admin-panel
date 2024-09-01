use super::{
    common::{prelude::*, Entity, EntityValue, GetListResponse, Unit},
    plugins::common::PluginEntities,
};
use crate::{
    apisix::{
        base::{PropertyType, Required},
        common::EntityFields,
    },
    macros::derive_common_default,
    proxy::ProxyFetchOpts,
};
use serde_json::{json, Value};
use std::collections::HashMap;

derive_common_default! {
pub struct RouteValue(pub EntityValue);}

pub type Route = Unit<RouteValue>;

impl EntityItemTrait for Route {
    const API_PREFIX: &'static str = "/routes";
    const DISPLAY_SHORT: &'static [&'static str] = &["uri"];
    const DOCS_KEY: &'static str = "route";
    const PLUGIN_ENTITY: Option<PluginEntities> = Some(PluginEntities::Route);

    entity_trait_get_value!();
}

pub type GetRoutesResponse = GetListResponse<Route>;
pub type RouteEntity = Entity<Route>;

impl RouteEntity {
    fn get_parsed_values(&self) -> (String, HashMap<String, Value>) {
        let mut new_route_values = self.parsed.value.0.get_cloned();
        let id = self.parsed.value.0.get_str("id");

        let existing_uri = new_route_values
            .get("uri")
            .map(|v| v.as_str().unwrap())
            .unwrap_or("");

        let parsed_uri = format!("/{}", existing_uri.trim_start_matches('/'));
        new_route_values.insert("uri".into(), json!(parsed_uri));
        new_route_values.remove("id");

        (id, new_route_values)
    }
}

impl EntityTrait for RouteEntity {
    fn create(&self) -> Result<ProxyFetchOpts, String> {
        let (id, mut new_route_values) = self.get_parsed_values();

        let existing_plugins = new_route_values
            .get("plugins")
            .unwrap_or(&json!({}))
            .clone();

        new_route_values.insert(
            "plugins".into(),
            serde_json::to_value(existing_plugins).unwrap(),
        );

        let data = serde_json::to_string(&new_route_values).ok();
        let (uri, method) = EntityValue::common_create(Route::API_PREFIX, id);

        Ok(ProxyFetchOpts { uri, method, data })
    }

    fn update(&self) -> Result<ProxyFetchOpts, String> {
        let (id, new_route_values) = self.get_parsed_values();

        let data = serde_json::to_string(&new_route_values).ok();
        let uri = format!("{}/{}", Route::API_PREFIX, id);

        Ok(ProxyFetchOpts {
            uri,
            method: crate::proxy::ProxyFetchMethod::PATCH,
            data,
        })
    }

    fn value_fields() -> Vec<EntityFields> {
        vec![
            EntityFields {
                description: "Unique text within the routes".to_string(),
                name: "id".to_string(),
                is_editable: false,
                ..Default::default()
            },
            EntityFields {
                description: "Identifier for the Route.".to_string(),
                name: "name".to_string(),
                ..Default::default()
            },
            EntityFields {
                description: "Description of usage scenarios.".to_string(),
                name: "desc".to_string(),
                ..Default::default()
            },
            EntityFields {
                description: "Configuration of the bound Service.".to_string(),
                name: "service_id".to_string(),
                ..Default::default()
            },
            EntityFields {
                description: "Matches with domain names such as foo.com or PAN domain names like *.foo.com.".to_string(),
                name: "hosts".to_string(),
                property_type: PropertyType::List(Box::new(PropertyType::String)),
                ..Default::default()
            },
            EntityFields {
                description: "Matches with the specified methods. Matches all methods if empty or unspecified.".to_string(),
                name: "methods".to_string(),
                property_type: PropertyType::List(Box::new(PropertyType::String)),
                ..Default::default()
            },
            EntityFields {
                description: "Id of the Upstream service.".to_string(),
                name: "upstream_id".to_string(),
                ..Default::default()
            },
            EntityFields {
                description: "Configuration of the Upstream.".to_string(),
                name: "upstream".to_string(),
                property_type: PropertyType::JSON,
                ..Default::default()
            },
            EntityFields {
                description: "Attributes of the Route specified as key-value pairs.".to_string(),
                example: Some(r#"{"version":"v2","build":"16","env":"production"}"#.to_string()),
                name: "labels".to_string(),
                property_type: PropertyType::JSON,
                ..Default::default()
            },
            EntityFields {
                description: "Enables a websocket. Set to false by default.".to_string(),
                name: "status".to_string(),
                property_type: PropertyType::Number,
                ..Default::default()
            },
            EntityFields {
                description: "Matches the uri. For more advanced matching see Router.".to_string(),
                is_required: Required::TrueIfOtherMissing(vec!["uris".to_string()]),
                name: "uri".to_string(),
                ..Default::default()
            },
            EntityFields {
                name: "plugins".to_string(),
                property_type: PropertyType::Plugins,
                ..Default::default()
            },
        ]
    }
}
