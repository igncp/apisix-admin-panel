use crate::{
    macros::derive_common_default,
    proxy::{ProxyFetchMethod, ProxyFetchOpts},
};

derive_common_default! {
pub struct ControlPlane;}

impl ControlPlane {
    pub fn get_schema() -> Result<ProxyFetchOpts, String> {
        ProxyFetchOpts::get("/v1/schema")
    }
    pub fn get_health_check() -> Result<ProxyFetchOpts, String> {
        ProxyFetchOpts::get("/v1/healthcheck")
    }
    pub fn reload_plugins() -> Result<ProxyFetchOpts, String> {
        Ok(ProxyFetchOpts {
            uri: "/v1/plugins/reload".to_string(),
            method: ProxyFetchMethod::PUT,
            data: None,
        })
    }
}
