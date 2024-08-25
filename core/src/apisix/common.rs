use std::collections::HashMap;

use crate::macros::derive_common_default;
use serde_json::Value;
use ts_rs::TS;

pub type OtherFields = HashMap<String, Value>;

derive_common_default! {
pub struct EntityValue {
    pub other_fields: Option<OtherFields>,
}}

pub const DISPLAY_SEPARATOR: &str = " | ";

impl EntityValue {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.other_fields
            .as_ref()
            .and_then(|fields| fields.get(key))
    }

    pub fn get_str(&self, key: &str) -> String {
        let item = self.get(key);

        item.map_or_else(|| "".to_string(), |v| v.as_str().unwrap().to_string())
    }

    pub fn get_cloned(&self) -> HashMap<String, Value> {
        self.other_fields.clone().unwrap_or_default()
    }

    pub fn init_if_empty(&mut self) {
        if self.other_fields.is_none() {
            self.other_fields = Some(HashMap::new());
        }
    }

    pub fn insert(&mut self, key: String, value: Value) {
        self.init_if_empty();
        self.other_fields.as_mut().unwrap().insert(key, value);
    }

    pub fn remove(&mut self, key: String) {
        self.init_if_empty();
        self.other_fields.as_mut().unwrap().remove(&key);
    }
}

derive_common_default! {
pub struct Unit<A> {
    #[serde(rename = "createdIndex")]
    pub created_index: u64,
    pub key: String,
    #[serde(rename = "modifiedIndex")]
    pub modified_index: u64,
    pub value: A,
}}

derive_common_default! {
pub struct GetListResponseBase<A> {
    pub list: Vec<A>,
    pub total: u32,
}}

pub type GetListResponse<A> = GetListResponseBase<Entity<A>>;

derive_common_default! {
#[derive(TS, Hash, Eq, PartialEq)]
#[ts(export)]
pub enum PropertyType {
    Boolean,
    List(Box<PropertyType>),
    Number,
    Plugins,
    #[default]
    String,
    Value,
}}

derive_common_default! {
#[derive(TS)]
#[ts(export)]
pub enum Required {
    True,
    #[default]
    False,
    TrueIfOtherMissing(Vec<String>),
}}

derive_common_default! {
#[derive(TS)]
#[ts(export)]
pub struct EntityFields {
    pub default_value: Option<String>,
    pub description: String,
    pub example: Option<String>,
    pub hidden: bool,
    pub is_required: Required,
    pub name: String,
    pub property_type: PropertyType,
}}

derive_common_default! {
pub struct Entity<A> {
    pub parsed: A,
    pub text: String,
}}
