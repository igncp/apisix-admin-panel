use apisix_admin_panel_core::apisix::plugins::{
    basic_auth::{ConsumerBasicAuthPlugin, RouteBasicAuthPlugin},
    consumer_restriction::ConsumerRestrictionPlugin,
    ip_restriction::IpRestrictionPlugin,
    key_auth::{ConsumerKeyAuthPlugin, RouteKeyAuthPlugin},
    limit_count::RouteLimitCountPlugin,
    proxy_rewrite::ProxyRewritePlugin,
    public_api::PublicApiPlugin,
    response_rewrite::ResponseRewritePlugin,
};
use serde::Serialize;
use serde_json::json;
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub struct WasmPluginDefinitions;

#[wasm_bindgen]
impl WasmPluginDefinitions {
    pub fn print() -> JsValue {
        let definitions = json!([
            ConsumerBasicAuthPlugin::new(),
            ConsumerKeyAuthPlugin::new(),
            ConsumerRestrictionPlugin::new(),
            IpRestrictionPlugin::new(),
            ProxyRewritePlugin::new(),
            PublicApiPlugin::new(),
            ResponseRewritePlugin::new(),
            RouteBasicAuthPlugin::new(),
            RouteKeyAuthPlugin::new(),
            RouteLimitCountPlugin::new()
        ]);

        let serializer = Serializer::new().serialize_maps_as_objects(true);

        definitions.serialize(&serializer).unwrap()
    }
}
