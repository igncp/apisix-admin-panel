macro_rules! derive_common {
    ($i:item) => {
        #[derive(
            Clone, Debug, serde::Serialize, serde::Deserialize, Default, derive_more::From,
        )]
        #[wasm_bindgen::prelude::wasm_bindgen]
        $i
    };
}

pub(crate) use derive_common;

macro_rules! entity_impl {
    ($structname: ident, $entity_struct: ident) => {
        use apisix_admin_panel_core::apisix::common::prelude::*;

        #[wasm_bindgen::prelude::wasm_bindgen]
        impl $structname {
            #[wasm_bindgen(constructor)]
            pub fn new() -> Self {
                Self::default()
            }

            #[wasm_bindgen(getter)]
            pub fn key(&self) -> Option<String> {
                self.0.parsed.key.clone()
            }

            #[wasm_bindgen(getter)]
            pub fn docs_key() -> String {
                $entity_struct::DOCS_KEY.to_string()
            }

            #[wasm_bindgen(getter)]
            pub fn plugin_entity() -> wasm_bindgen::JsValue {
                serde_wasm_bindgen::to_value(&$entity_struct::PLUGIN_ENTITY).unwrap()
            }

            pub fn short_display_base(&self) -> String {
                let id = self.0.parsed.value.0.get_str("id");
                let name = self.0.parsed.value.0.get_str("name");
                let others = $entity_struct::DISPLAY_SHORT
                    .iter()
                    .map(|f| self.0.parsed.value.0.get_str(f))
                    .filter(|v| !v.is_empty())
                    .collect::<Vec<String>>()
                    .join(" | ");

                let ideal_value = [name, others]
                    .iter()
                    .cloned()
                    .filter(|v| !v.is_empty())
                    .collect::<Vec<String>>()
                    .join(" | ");

                if ideal_value.is_empty() {
                    id
                } else {
                    ideal_value
                }
            }

            #[wasm_bindgen(getter)]
            pub fn short_display(&self) -> wasm_bindgen::JsValue {
                serde_wasm_bindgen::to_value(&self.short_display_base()).unwrap()
            }

            #[wasm_bindgen(getter)]
            pub fn long_display(&self) -> wasm_bindgen::JsValue {
                let short_display = self.short_display_base();
                let desc = self.0.parsed.value.0.get_str("desc");
                let result = [short_display, desc]
                    .iter()
                    .cloned()
                    .filter(|v| !v.is_empty())
                    .collect::<Vec<String>>()
                    .join(" | ");

                serde_wasm_bindgen::to_value(&result).unwrap()
            }

            #[wasm_bindgen(getter)]
            pub fn text(&self) -> String {
                self.0.text.clone()
            }

            pub fn get_all() -> Result<crate::proxy::WasmProxyFetchOpts, String> {
                apisix_admin_panel_core::proxy::ProxyFetchOpts::get($entity_struct::API_PREFIX)
                    .map(Into::into)
            }

            pub fn delete(&self) -> Result<crate::proxy::WasmProxyFetchOpts, String> {
                self.0.delete().map(Into::into)
            }

            pub fn create(&self) -> Result<crate::proxy::WasmProxyFetchOpts, String> {
                self.0.create().map(Into::into)
            }

            pub fn update(&self) -> Result<crate::proxy::WasmProxyFetchOpts, String> {
                self.0.update().map(Into::into)
            }
        }
    };
}

pub(crate) use entity_impl;

macro_rules! entity_list_impl {
    ($structname: ident, $entity_name: ident) => {
        #[wasm_bindgen::prelude::wasm_bindgen]
        impl $structname {
            #[wasm_bindgen::prelude::wasm_bindgen(getter)]
            pub fn list(&self) -> Vec<$entity_name> {
                self.0.list.iter().map(|c| c.clone().into()).collect()
            }

            #[wasm_bindgen(getter)]
            pub fn total(&self) -> u32 {
                self.0.total
            }
        }

        impl $entity_name {
            fn check_error_msg(response: wasm_bindgen::prelude::JsValue) -> Result<(), String> {
                let str_result = serde_wasm_bindgen::from_value::<String>(response);

                if let Ok(e) = str_result {
                    #[derive(serde::Deserialize)]
                    struct ErrorResponse {
                        error_msg: String,
                    }

                    if let Ok(e) = serde_json::from_str::<ErrorResponse>(&e) {
                        return Err(e.error_msg);
                    }
                }

                Ok(())
            }
        }

        #[wasm_bindgen::prelude::wasm_bindgen]
        impl $entity_name {
            pub fn create_response(response: wasm_bindgen::prelude::JsValue) -> Result<(), String> {
                Self::check_error_msg(response)
            }

            pub fn delete_response(response: wasm_bindgen::prelude::JsValue) -> Result<(), String> {
                Self::check_error_msg(response)
            }
        }
    };
}

pub(crate) use entity_list_impl;

macro_rules! entity_fields_impl {
    ($wasm_struct: ident, $base_struct: ident, $wasm_get_all: ident, $core_struct: ident) => {
        #[wasm_bindgen::prelude::wasm_bindgen]
        impl $wasm_struct {
            pub fn set_field(&mut self, key: String, val: wasm_bindgen::prelude::JsValue) {
                let json_val: serde_json::Value = serde_wasm_bindgen::from_value(val).unwrap();

                self.0.parsed.value.0.insert(key, json_val);
            }

            pub fn set_plugins(&mut self, val: wasm_bindgen::prelude::JsValue) {
                let json_val: Result<serde_json::Value, _> = serde_wasm_bindgen::from_value(val);

                if json_val.is_err() {
                    self.0.parsed.value.0.remove("plugins".to_string());
                    return;
                }

                let json_val = json_val.unwrap();

                if json_val.is_null() {
                    self.0.parsed.value.0.remove("plugins".to_string());
                    return;
                }

                self.0
                    .parsed
                    .value
                    .0
                    .insert("plugins".to_string(), json_val);
            }

            pub fn get_field(&mut self, key: String) -> wasm_bindgen::prelude::JsValue {
                let json_val = self.0.parsed.value.0.get(&key).clone();

                serde_wasm_bindgen::to_value(&json_val).unwrap()
            }

            pub fn fields_definitions() -> wasm_bindgen::prelude::JsValue {
                serde_wasm_bindgen::to_value(&$base_struct::value_fields()).unwrap()
            }

            pub fn get_all_response(
                response: wasm_bindgen::prelude::JsValue,
            ) -> Result<$wasm_get_all, String> {
                let list_of_unknowns: crate::apisix::common::GetListUnknowns =
                    serde_wasm_bindgen::from_value(response)
                        .map_err(|_| "Error parsing list".to_string())?;

                let new_list = list_of_unknowns
                    .list
                    .iter()
                    .map(|s| {
                        let text = serde_json::to_string(s)
                            .map_err(|_| "Error serializing".to_string())?;
                        let mut parsed: $core_struct = serde_json::from_str(&text)
                            .map_err(|e| format!("Error parsing item {:?}", e).to_string())?;
                        let value: serde_json::Value = s
                            .get("value")
                            .map(|v| v.clone())
                            .unwrap_or(serde_json::json!({}));
                        let parsed_value = serde_json::from_value(value.clone())
                            .map_err(|_| "Error parsing value".to_string())?;

                        parsed.value.0.other_fields = parsed_value;

                        Ok(apisix_admin_panel_core::apisix::common::Entity { parsed, text })
                    })
                    .collect::<Result<Vec<$base_struct>, String>>()?;

                Ok($wasm_get_all(
                    apisix_admin_panel_core::apisix::common::GetListResponse {
                        list: new_list,
                        total: list_of_unknowns.total,
                    },
                ))
            }

            pub fn add_extra_json(&mut self, extra_json: wasm_bindgen::prelude::JsValue) {
                let extra_json: std::collections::HashMap<String, serde_json::Value> =
                    serde_wasm_bindgen::from_value(extra_json).unwrap();

                self.0.parsed.value.0.merge_json(extra_json);
            }
        }
    };
}

pub(crate) use entity_fields_impl;
