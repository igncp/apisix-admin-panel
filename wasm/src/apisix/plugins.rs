use apisix_admin_panel_core::apisix::plugins::{
    basic_auth::{ConsumerBasicAuthPlugin, RouteBasicAuthPlugin},
    key_auth::{ConsumerKeyAuthPlugin, RouteKeyAuthPlugin},
    limit_count::RouteLimitCountPlugin,
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
            RouteBasicAuthPlugin::new(),
            RouteKeyAuthPlugin::new(),
            RouteLimitCountPlugin::new()
        ]);

        let serializer = Serializer::new().serialize_maps_as_objects(true);

        definitions.serialize(&serializer).unwrap()
    }
}
