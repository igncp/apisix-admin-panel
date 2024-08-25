use apisix_admin_panel_core::apisix::common::{Entity, GetListResponse, GetListResponseBase};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;
use wasm_bindgen::JsValue;

pub type GetListUnknowns = GetListResponseBase<HashMap<String, Value>>;

pub fn convert_list_response<A: DeserializeOwned>(response: JsValue) -> Option<GetListResponse<A>> {
    let list_of_unknowns: GetListUnknowns = serde_wasm_bindgen::from_value(response).ok()?;

    let new_list = list_of_unknowns
        .list
        .iter()
        .map(|s| {
            let text = serde_json::to_string(s).unwrap();
            let parsed: A = serde_json::from_str(&text).unwrap();

            Entity { parsed, text }
        })
        .collect::<Vec<Entity<A>>>();

    Some(GetListResponse {
        list: new_list,
        total: list_of_unknowns.total,
    })
}
