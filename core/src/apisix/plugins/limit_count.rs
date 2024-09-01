use super::common::{PluginDefinition, PluginEntities, PluginOption};
use crate::{
    apisix::base::{PropertyType, Required},
    macros::derive_common_default,
};

derive_common_default! {
pub struct LimitCountPlugin(PluginDefinition);}

impl LimitCountPlugin {
    pub fn new() -> Self {
        Self(PluginDefinition {
            name: "limit-count".to_string(),
            entities: [
                PluginEntities::Route,
                PluginEntities::Consumer,
                PluginEntities::ConsumerGroup,
            ]
            .iter()
            .cloned()
            .collect(),
            options: vec![
                PluginOption {
                    description: "Maximum number of requests to allow.".to_string(),
                    is_required: Required::True,
                    name: "count".to_string(),
                    property_type: PropertyType::Number,
                    ..Default::default()
                },
                PluginOption {
                    description: "Time in seconds before count is reset.".to_string(),
                    is_required: Required::True,
                    name: "time_window".to_string(),
                    property_type: PropertyType::Number,
                    ..Default::default()
                },
                PluginOption {
                    description: "Type of user specified key to use.".to_string(),
                    name: "key_type".to_string(),
                    property_type: PropertyType::Enum(vec![
                        "var".to_string(),
                        "var_combination".to_string(),
                        "constant".to_string(),
                    ]),
                    ..Default::default()
                },
                PluginOption {
                    description: "User specified key to base the request limiting on. If the key_type attribute is set to constant, the key will be treated as a constant value. If the key_type attribute is set to var, the key will be treated as a name of variable, like remote_addr or consumer_name. If the key_type is set to var_combination, the key will be a combination of variables, like $remote_addr $consumer_name. If the value of the key is empty, remote_addr will be set as the default key.".to_string(),
                    name: "key".to_string(),
                    ..Default::default()
                },
                PluginOption {
                    description: "HTTP status code returned when the requests exceeding the threshold are rejected.".to_string(),
                    name: "rejected_code".to_string(),
                    property_type: PropertyType::Number,
                    ..Default::default()
                },
                PluginOption {
                    description: "non-empty	Body of the response returned when the requests exceeding the threshold are rejected.".to_string(),
                    name: "rejected_msg".to_string(),
                    ..Default::default()
                },
                PluginOption {
                    description: "Rate-limiting policies to use for retrieving and increment the limit count. When set to local the counters will be locally stored in memory on the node. When set to redis counters are stored on a Redis server and will be shared across the nodes. It is done usually for global speed limiting, and setting to redis-cluster uses a Redis cluster instead of a single instance.".to_string(),
                    name: "policy".to_string(),
                    property_type: PropertyType::Enum(vec![
                        "local".to_string(),
                        "redis".to_string(),
                        "redis-cluster".to_string(),
                    ]),
                    ..Default::default()
                },
                PluginOption {
                    description: "When set to true enables Plugin degradation when the Plugin is temporarily unavailable (for example, a Redis timeout) and allows requests to continue.".to_string(),
                    name: "allow_degradation".to_string(),
                    property_type: PropertyType::Boolean,
                    ..Default::default()
                },
                PluginOption {
                    description: "When set to true, adds X-RateLimit-Limit (total number of requests) and X-RateLimit-Remaining (remaining number of requests) to the response header.".to_string(),
                    name: "show_limit_quota_header".to_string(),
                    property_type: PropertyType::Boolean,
                    ..Default::default()
                },
                PluginOption {
                    description: "Group to share the counter with. Routes configured with the same group will share the same counter. Do not configure with a value that was previously used in this attribute before as the plugin would not allow.".to_string(),
                    name: "group".to_string(),
                    ..Default::default()
                },
                PluginOption {
                    description: "Address of the Redis server. Used when the policy attribute is set to redis.".to_string(),
                    name: "redis_host".to_string(),
                    ..Default::default()
                },
                PluginOption {
                    description: "Port of the Redis server. Used when the policy attribute is set to redis.".to_string(),
                    name: "redis_port".to_string(),
                    property_type: PropertyType::Number,
                    ..Default::default()
                },
                PluginOption {
                    description: "Username for Redis authentication if Redis ACL is used (for Redis version >= 6.0). If you use the legacy authentication method requirepass to configure Redis password, configure only the redis_password. Used when the policy is set to redis.".to_string(),
                    name: "redis_username".to_string(),
                    ..Default::default()
                },
                PluginOption {
                    description: "Password for Redis authentication. Used when the policy is set to redis or redis-cluster.".to_string(),
                    name: "redis_password".to_string(),
                    ..Default::default()
                },
                PluginOption {
                    description: "If set to true, then uses SSL to connect to redis instance. Used when the policy attribute is set to redis.".to_string(),
                    name: "redis_ssl".to_string(),
                    property_type: PropertyType::Boolean,
                    ..Default::default()
                },
                PluginOption {
                    description: "If set to true, then verifies the validity of the server SSL certificate. Used when the policy attribute is set to redis. See tcpsock:sslhandshake.".to_string(),
                    name: "redis_ssl_verify".to_string(),
                    property_type: PropertyType::Boolean,
                    ..Default::default()
                },
                PluginOption {
                    description: "Selected database of the Redis server (for single instance operation or when using Redis cloud with a single entrypoint). Used when the policy attribute is set to redis.".to_string(),
                    name: "redis_database".to_string(),
                    property_type: PropertyType::Number,
                    ..Default::default()
                },
                PluginOption {
                    description: "Timeout in milliseconds for any command submitted to the Redis server. Used when the policy attribute is set to redis or redis-cluster.".to_string(),
                    name: "redis_timeout".to_string(),
                    property_type: PropertyType::Number,
                    ..Default::default()
                },
                PluginOption {
                    description: "Addresses of Redis cluster nodes. Used when the policy attribute is set to redis-cluster.".to_string(),
                    name: "redis_cluster_nodes".to_string(),
                    property_type: PropertyType::List(Box::new(PropertyType::String)),
                    ..Default::default()
                },
                PluginOption {
                    description: "Name of the Redis cluster service nodes. Used when the policy attribute is set to redis-cluster.".to_string(),
                    name: "redis_cluster_name".to_string(),
                    ..Default::default()
                },
                PluginOption {
                    description: "If set to true, then uses SSL to connect to redis-cluster. Used when the policy attribute is set to redis-cluster.".to_string(),
                    name: "redis_cluster_ssl".to_string(),
                    property_type: PropertyType::Boolean,
                    ..Default::default()
                },
                PluginOption {
                    description: "If set to true, then verifies the validity of the server SSL certificate. Used when the policy attribute is set to redis-cluster.".to_string(),
                    name: "redis_cluster_ssl_verify".to_string(),
                    property_type: PropertyType::Boolean,
                    ..Default::default()
                },
            ],
        })
    }
}
