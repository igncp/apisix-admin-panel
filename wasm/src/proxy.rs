use crate::macros::derive_common;
use apisix_admin_panel_core::proxy::ProxyFetchOpts;
use wasm_bindgen::prelude::*;

derive_common! {
pub struct WasmProxyFetchOpts(ProxyFetchOpts);}

#[wasm_bindgen]
impl WasmProxyFetchOpts {
    pub fn format(&self) -> String {
        serde_json::to_string(&self.0).unwrap()
    }
}
