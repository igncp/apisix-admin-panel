use crate::{macros::derive_common, proxy::WasmProxyFetchOpts};
use apisix_admin_panel_core::apisix::control_plane::ControlPlane;
use wasm_bindgen::prelude::*;

derive_common! {
pub struct WasmControlPlane(ControlPlane);}

#[wasm_bindgen]
impl WasmControlPlane {
    pub fn get_health_check() -> Result<WasmProxyFetchOpts, String> {
        Ok(ControlPlane::get_health_check()?.into())
    }

    pub fn get_schema() -> Result<WasmProxyFetchOpts, String> {
        Ok(ControlPlane::get_schema()?.into())
    }

    pub fn reload_plugins() -> Result<WasmProxyFetchOpts, String> {
        Ok(ControlPlane::reload_plugins()?.into())
    }
}
