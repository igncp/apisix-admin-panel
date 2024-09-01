use super::common::{PluginDefinition, PluginEntities, PluginOption};
use crate::{
    apisix::base::{PropertyType, Required},
    macros::derive_common_default,
};

derive_common_default! {
pub struct ProxyMirrorPlugin(PluginDefinition);}

impl ProxyMirrorPlugin {
    pub fn new() -> Self {
        Self(PluginDefinition {
            name: "proxy-mirror".to_string(),
            entities: [PluginEntities::Route, PluginEntities::Service]
                .iter()
                .cloned()
                .collect(),
                options: vec![
                    PluginOption {
                        description: "Address of the mirror service. It needs to contain the scheme (http(s) or grpc(s)) but without the path. For example, http://127.0.0.1:9797.".to_string(),
                        name: "host".to_string(),
                        is_required: Required::True,
                        ..Default::default()
                    },
                    PluginOption {
                        description: "Path of the mirror request. If unspecified, current path will be used. If it is for mirroring grpc traffic, this option is no longer applicable.".to_string(),
                        name: "path".to_string(),
                        ..Default::default()
                    },
                    PluginOption {
                        description: r#"If the path of a mirror request is specified, set the concatenation mode of request paths. The replace mode will directly use path as the path of the mirror request. The prefix mode will use the path + source request URI as the path to the mirror request. If it is for mirroring grpc traffic, this option is no longer applicable too."#.to_string(),
                        name: "path_concat_mode".to_string(),
                        ..Default::default()
                    },
                    PluginOption {
                        description: "Ratio of the requests that will be mirrored.".to_string(),
                        name: "sample_ratio".to_string(),
                        property_type: PropertyType::Number,
                        ..Default::default()
                    }
                ],
        })
    }
}
