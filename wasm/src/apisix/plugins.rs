use apisix_admin_panel_core::apisix::plugins::{
    basic_auth::{ConsumerBasicAuthPlugin, RouteBasicAuthPlugin},
    consumer_restriction::ConsumerRestrictionPlugin,
    cors::CorsPlugin,
    ext_plugin_pre_req::ExtPluginPreReqPlugin,
    ip_restriction::IpRestrictionPlugin,
    key_auth::{ConsumerKeyAuthPlugin, RouteKeyAuthPlugin},
    limit_count::LimitCountPlugin,
    prometheus::PrometheusPlugin,
    proxy_mirror::ProxyMirrorPlugin,
    proxy_rewrite::ProxyRewritePlugin,
    public_api::PublicApiPlugin,
    response_rewrite::ResponseRewritePlugin,
    uri_blocker::UriBlockerPlugin,
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
            CorsPlugin::new(),
            ExtPluginPreReqPlugin::new(),
            IpRestrictionPlugin::new(),
            LimitCountPlugin::new(),
            PrometheusPlugin::new(),
            ProxyMirrorPlugin::new(),
            ProxyRewritePlugin::new(),
            PublicApiPlugin::new(),
            ResponseRewritePlugin::new(),
            RouteBasicAuthPlugin::new(),
            RouteKeyAuthPlugin::new(),
            UriBlockerPlugin::new()
        ]);

        let serializer = Serializer::new().serialize_maps_as_objects(true);

        definitions.serialize(&serializer).unwrap()
    }
}
