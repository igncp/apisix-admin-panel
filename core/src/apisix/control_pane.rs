use crate::{macros::derive_common_default, proxy::ProxyFetchOpts};

derive_common_default! {
pub struct ControlPane;}

impl ControlPane {
    pub fn get_schema() -> Result<ProxyFetchOpts, String> {
        ProxyFetchOpts::get("/v1/schema")
    }
    pub fn get_health_check() -> Result<ProxyFetchOpts, String> {
        ProxyFetchOpts::get("/v1/healthcheck")
    }
}
