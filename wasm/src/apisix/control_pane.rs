use crate::{macros::derive_common, proxy::WasmProxyFetchOpts};
use apisix_admin_panel_core::apisix::control_pane::ControlPane;
use wasm_bindgen::prelude::*;

derive_common! {
pub struct WasmControlPane(ControlPane);}

#[wasm_bindgen]
impl WasmControlPane {
    pub fn get_schema() -> Result<WasmProxyFetchOpts, String> {
        Ok(ControlPane::get_schema()?.into())
    }

    pub fn get_health_check() -> Result<WasmProxyFetchOpts, String> {
        Ok(ControlPane::get_health_check()?.into())
    }

    pub fn reload_plugins() -> Result<WasmProxyFetchOpts, String> {
        Ok(ControlPane::reload_plugins()?.into())
    }
}
