use super::common::{PluginDefinition, PluginEntities, PluginOption};
use crate::{apisix::base::PropertyType, macros::derive_common_default};

derive_common_default! {
pub struct CorsPlugin(PluginDefinition);}

impl CorsPlugin {
    pub fn new() -> Self {
        Self(PluginDefinition {
            name: "cors".to_string(),
            entities: [PluginEntities::Route, PluginEntities::Consumer, PluginEntities::ConsumerGroup, PluginEntities::Service].iter().cloned().collect(),
            options: vec![
                PluginOption {
                    description: "Origins to allow CORS. Use the scheme://host:port format. For example, https://somedomain.com:8081. If you have multiple origins, use a , to list them. If allow_credential is set to false, you can enable CORS for all origins by using *. If allow_credential is set to true, you can forcefully allow CORS on all origins by using ** but it will pose some security issues.".to_string(),
                    name: "allow_origins".to_string(),
                    ..Default::default()
                },
                PluginOption {
                    description: "Request methods to enable CORS on. For example GET, POST. Use , to add multiple methods. If allow_credential is set to false, you can enable CORS for all methods by using *. If allow_credential is set to true, you can forcefully allow CORS on all methods by using ** but it will pose some security issues.".to_string(),
                    name: "allow_methods".to_string(),
                    ..Default::default()
                },
                PluginOption {
                    description: "Headers in the request allowed when accessing a cross-origin resource. Use , to add multiple headers. If allow_credential is set to false, you can enable CORS for all request headers by using *. If allow_credential is set to true, you can forcefully allow CORS on all request headers by using ** but it will pose some security issues.".to_string(),
                    name: "allow_headers".to_string(),
                    ..Default::default()
                },
                PluginOption {
                    description: "Headers in the response allowed when accessing a cross-origin resource. Use , to add multiple headers. If allow_credential is set to false, you can enable CORS for all response headers by using *. If not specified, the plugin will not modify the Access-Control-Expose-Headers header. See Access-Control-Expose-Headers - MDN for more details.".to_string(),
                    name: "expose_headers".to_string(),
                    ..Default::default()
                },
                PluginOption {
                    description: "False	5	Maximum time in seconds the result is cached. If the time is within this limit, the browser will check the cached result. Set to -1 to disable caching. Note that the maximum value is browser dependent. See Access-Control-Max-Age for more details.".to_string(),
                    name: "max_age".to_string(),
                    property_type: PropertyType::Number,
                    ..Default::default()
                },
                PluginOption {
                    description: "false	When set to true, allows requests to include credentials like cookies. According to CORS specification, if you set this to true, you cannot use '*' to allow all for the other attributes.".to_string(),
                    name: "allow_credential".to_string(),
                    property_type: PropertyType::Boolean,
                    ..Default::default()
                },
                PluginOption {
                    description: r#"Regex to match origins that allow CORS. For example, [".*\.test.com$"] can match all subdomains of test.com. When set to specified range, only domains in this range will be allowed, no matter what allow_origins is."#.to_string(),
                    name: "allow_origins_by_regex".to_string(),
                    property_type: PropertyType::List(
                        Box::new(PropertyType::String),
                    ),
                    ..Default::default()
                },
                PluginOption {
                    description: r#"Origins to enable CORS referenced from allow_origins set in the Plugin metadata. For example, if "allow_origins": {"EXAMPLE": "https://example.com"} is set in the Plugin metadata, then ["EXAMPLE"] can be used to allow CORS on the origin https://example.com."#.to_string(),
                    name: "allow_origins_by_metadata".to_string(),
                    property_type: PropertyType::List(
                        Box::new(PropertyType::String),
                    ),
                    ..Default::default()
                }
            ],
        })
    }
}
