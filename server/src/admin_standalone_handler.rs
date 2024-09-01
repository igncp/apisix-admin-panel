use actix_files::NamedFile;
use apisix_admin_panel_core::{
    apisix::{
        common::prelude::*, consumer_groups::ConsumerGroup, consumers::Consumer, routes::Route,
        services::Service, upstreams::Upstream,
    },
    proxy::{ProxyFetchMethod, ProxyFetchOpts},
};
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    io::{Read, Write},
};

pub struct AdminStandaloneHandler {
    pub config_path: String,
}

impl AdminStandaloneHandler {
    pub async fn handle(&self, opts: ProxyFetchOpts) -> Result<String, &'static str> {
        let mut file = NamedFile::open(self.config_path.clone())
            .map_err(|_| "APISIX config file not found")?;

        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|_| "Failed to read APISIX config file")?;

        let parsed_content: serde_yaml::Value =
            serde_yaml::from_str(&content).map_err(|_| "Failed to parse APISIX config file")?;

        let json_content: Value = serde_json::to_value(&parsed_content)
            .map_err(|_| "Failed to convert APISIX config to JSON")?;

        let map_content: HashMap<String, Value> = serde_json::from_value(json_content)
            .map_err(|_| "Failed to convert APISIX config to JSON")?;

        if opts.uri.starts_with(Route::API_PREFIX) {
            return self.handle_items("routes", opts, map_content).await;
        } else if opts.uri.starts_with(Consumer::API_PREFIX) {
            return self.handle_items("consumers", opts, map_content).await;
        } else if opts.uri.starts_with(Service::API_PREFIX) {
            return self.handle_items("services", opts, map_content).await;
        } else if opts.uri.starts_with(Upstream::API_PREFIX) {
            return self.handle_items("upstreams", opts, map_content).await;
        } else if opts.uri.starts_with(ConsumerGroup::API_PREFIX) {
            return self
                .handle_items("consumer-groups", opts, map_content)
                .await;
        }

        let json_str = serde_json::to_string(&map_content)
            .map_err(|_| "Failed to convert APISIX config to JSON")?;

        Ok(json_str)
    }

    async fn write_file(&self, map_content: HashMap<String, Value>) -> Result<(), &'static str> {
        let new_yaml = serde_yaml::to_string(&map_content)
            .map_err(|_| "Failed to convert APISIX config to YAML")?;
        let new_file = format!("{}\n#END", new_yaml);

        let mut file = std::fs::File::create(self.config_path.clone())
            .map_err(|_| "Failed to open APISIX config file")?;

        file.write_all(new_file.as_bytes())
            .map_err(|_| "Failed to write APISIX config")?;

        Ok(())
    }

    fn get_id_from_uri(uri: &str) -> Result<String, &'static str> {
        Ok(uri
            .split("/")
            .last()
            .ok_or("Item ID not found")?
            .to_string())
    }

    async fn handle_items(
        &self,
        item_type: &str,
        opts: ProxyFetchOpts,
        mut map_content: HashMap<String, Value>,
    ) -> Result<String, &'static str> {
        let id_key = match item_type {
            "consumers" => "username",
            _ => "id",
        };
        let mut items = map_content.get(item_type).unwrap_or(&json!([])).clone();
        if items.is_null() {
            items = json!([]);
        }

        let mut items_raw: Vec<HashMap<String, Value>> = serde_json::from_value(items).unwrap();

        let items = items_raw
            .iter_mut()
            .enumerate()
            .map(|(item_raw_idx, item_raw)| {
                let current_plugins = item_raw.get("plugins");
                if let Some(plugins) = current_plugins {
                    let mut plugins_map: HashMap<String, Value> =
                        serde_json::from_value(plugins.clone()).unwrap();

                    for (plugin_name, plugin) in plugins_map.clone().iter() {
                        if plugin.clone() == Value::Null {
                            let empty_object: HashMap<String, Value> = HashMap::new();
                            plugins_map.insert(plugin_name.clone(), json!(empty_object));
                        }
                    }

                    item_raw.insert("plugins".to_string(), json!(plugins_map));
                }
                let current_id = item_raw.get(id_key);
                if current_id.is_none() {
                    let item_id = item_raw_idx.to_string();
                    item_raw.insert(id_key.to_string(), json!(item_id));
                }

                let new_route = json!({
                    "value": item_raw.clone(),
                });

                new_route
            })
            .collect::<Vec<Value>>();

        if opts.method == ProxyFetchMethod::GET {
            let response = json!({
                "list": items,
                "total": items.len(),
            });

            return Ok(serde_json::to_string(&response).unwrap());
        } else if opts.method == ProxyFetchMethod::PATCH
            || opts.method == ProxyFetchMethod::PUT
            || opts.method == ProxyFetchMethod::POST
        {
            let mut updated_item: HashMap<String, Value> =
                serde_json::from_str(&opts.data.unwrap())
                    .map_err(|_| "Failed to parse route data")?;

            // This should be decided from core
            let item_id = match item_type {
                "consumers" => updated_item
                    .get(id_key)
                    .ok_or("Missing consumer")?
                    .as_str()
                    .ok_or("Consumer ID not found")?
                    .to_string(),
                _ => Self::get_id_from_uri(&opts.uri)?,
            };

            if opts.method == ProxyFetchMethod::PUT || opts.method == ProxyFetchMethod::PATCH {
                let previous_route = items_raw.iter_mut().find(|route| {
                    let current_id = route.get(id_key);
                    if current_id.is_none() {
                        return false;
                    }
                    let current_id = current_id.unwrap().as_str().unwrap();
                    current_id == item_id
                });

                if previous_route.is_none() && opts.method == ProxyFetchMethod::PATCH {
                    return Err("Item not found");
                }

                if let Some(previous_route) = previous_route {
                    for (key, _) in previous_route.clone().iter() {
                        previous_route.remove(key);
                    }

                    for (key, value) in updated_item.iter() {
                        previous_route.insert(key.clone(), value.clone());
                    }
                    let new_routes = json!(items_raw);
                    map_content.insert(item_type.to_string(), new_routes);

                    self.write_file(map_content).await?;

                    return Ok("{}".to_string());
                }
            }

            let plugins = updated_item.get("plugins");
            let id = match opts.method {
                ProxyFetchMethod::POST => "".to_string(),
                _ => {
                    if item_type == "consumers" {
                        updated_item
                            .get(id_key)
                            .ok_or("Item ID not found")?
                            .as_str()
                            .ok_or("Item ID not found")?
                            .to_string()
                    } else {
                        opts.uri
                            .split("/")
                            .last()
                            .ok_or("Item ID not found")?
                            .to_string()
                    }
                }
            };
            if let Some(plugins) = plugins {
                let plugins_map: HashMap<String, Value> =
                    serde_json::from_value(plugins.clone()).unwrap();

                if plugins_map.is_empty() {
                    updated_item.remove("plugins");
                }
            }
            if !id.is_empty() {
                updated_item.insert(id_key.to_string(), id.into());
            }
            items_raw.push(updated_item);
            let new_routes = json!(items_raw);
            map_content.insert(item_type.to_string(), new_routes);

            self.write_file(map_content).await?;
        } else if opts.method == ProxyFetchMethod::DELETE {
            let item_id = opts.uri.split("/").last().ok_or("Item ID not found")?;
            let routes_filtered = items_raw
                .iter()
                .filter(|route| {
                    let current_id = route.get(id_key);
                    if current_id.is_none() {
                        return false;
                    }
                    let current_id = current_id.unwrap().as_str().unwrap();
                    current_id != item_id
                })
                .collect::<Vec<&HashMap<String, Value>>>();

            let new_routes = json!(routes_filtered);
            map_content.insert(item_type.to_string(), new_routes);

            self.write_file(map_content).await?;
        }

        Ok("{}".to_string())
    }
}
