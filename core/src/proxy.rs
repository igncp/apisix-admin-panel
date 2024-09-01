use crate::macros::{derive_common, derive_common_default};

derive_common! {
#[derive(PartialEq, Eq)]
pub enum ProxyFetchMethod {
    DELETE,
    GET,
    PATCH,
    POST,
    PUT,
}}

impl Default for ProxyFetchMethod {
    fn default() -> Self {
        Self::GET
    }
}

derive_common_default! {
pub struct ProxyFetchOpts {
    pub uri: String,
    pub method: ProxyFetchMethod,
    pub data: Option<String>,
}}

impl ProxyFetchOpts {
    pub fn get(uri: &str) -> Result<Self, String> {
        Ok(Self {
            uri: uri.to_string(),
            method: ProxyFetchMethod::GET,
            data: None,
        })
    }
    pub fn del(uri: String) -> Result<Self, String> {
        Ok(Self {
            uri,
            method: ProxyFetchMethod::DELETE,
            data: None,
        })
    }
}
