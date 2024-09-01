use crate::{
    macros::{derive_common, derive_common_default},
    proxy::{ProxyFetchMethod, ProxyFetchOpts},
};
use serde_json::Value;
use std::collections::HashMap;
use ts_rs::TS;

use super::{
    base::{PropertyType, Required},
    plugins::common::PluginEntities,
};

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

    pub fn common_create(prefix: &str, id: String) -> (String, ProxyFetchMethod) {
        let uri = format!(
            "{}{}",
            prefix,
            if id.is_empty() {
                "".to_string()
            } else {
                format!("/{}", id)
            }
        );
        let method = if id.is_empty() {
            ProxyFetchMethod::POST
        } else {
            ProxyFetchMethod::PUT
        };

        (uri, method)
    }

    pub fn common_update(prefix: &str, id: String) -> (String, ProxyFetchMethod) {
        let uri = format!("{}/{}", prefix, id);
        let method = ProxyFetchMethod::PATCH;

        (uri, method)
    }
}

derive_common_default! {
pub struct Unit<A> {
    #[serde(rename = "createdIndex")]
    pub created_index: Option<u64>,
    pub key: Option<String>,
    #[serde(rename = "modifiedIndex")]
    pub modified_index: Option<u64>,
    pub value: A,
}}

derive_common_default! {
pub struct GetListResponseBase<A> {
    pub list: Vec<A>,
    pub total: u32,
}}

pub type GetListResponse<A> = GetListResponseBase<Entity<A>>;

derive_common! {
#[derive(TS)]
#[ts(export)]
pub struct EntityFields {
    pub default_value: Option<String>,
    pub description: String,
    pub example: Option<String>,
    pub hidden: bool,
    pub is_editable: bool,
    pub is_required: Required,
    pub name: String,
    pub property_type: PropertyType,
}}

impl Default for EntityFields {
    fn default() -> Self {
        Self {
            default_value: None,
            description: Default::default(),
            example: None,
            hidden: false,
            is_editable: true,
            is_required: Default::default(),
            name: Default::default(),
            property_type: Default::default(),
        }
    }
}

derive_common_default! {
pub struct Entity<A> {
    pub parsed: A,
    pub text: String,
}}

pub trait EntityValueTrait {
    fn get_str(&self, key: &str) -> String;
}

impl EntityValueTrait for EntityValue {
    fn get_str(&self, key: &str) -> String {
        let item = self.get(key);

        item.map_or_else(|| "".to_string(), |v| v.as_str().unwrap().to_string())
    }
}

pub trait EntityItemTrait {
    const API_PREFIX: &'static str;
    const DOCS_KEY: &'static str;
    const PLUGIN_ENTITY: Option<PluginEntities>;

    const DISPLAY_LONG: &'static [&'static str] = &[];
    const DISPLAY_SHORT: &'static [&'static str] = &[];
    const ID_NAME: &'static str = "id";
    const REQUIRED_VERSION: Option<&'static str> = None;

    type Value: EntityValueTrait;

    fn get_value(&self) -> &Self::Value;
    fn get_cloned(&self) -> HashMap<String, Value>;
}

pub trait EntityTrait {
    fn create(&self) -> Result<ProxyFetchOpts, String>;
    fn update(&self) -> Result<ProxyFetchOpts, String>;
    fn value_fields() -> Vec<EntityFields>;
}

impl<A: EntityItemTrait> Entity<A> {
    pub fn get_common_parsed_values(&self) -> (String, Option<String>) {
        let mut new_values = self.parsed.get_cloned();
        let id = self.parsed.get_value().get_str(A::ID_NAME);

        new_values.remove(A::ID_NAME);
        let opts = Some(serde_json::to_string(&new_values).unwrap());

        (id, opts)
    }

    pub fn check_id(&self) -> Result<String, String> {
        let id = self.parsed.get_value().get_str(A::ID_NAME);

        if id.is_empty() {
            Err("Id is required for delete".to_string())
        } else {
            Ok(id.to_string())
        }
    }

    pub fn delete(&self) -> Result<ProxyFetchOpts, String> {
        let id = self.check_id()?;
        let uri = format!("{}/{}", A::API_PREFIX, id);
        ProxyFetchOpts::del(uri)
    }
}

// Only include macros and traits here
pub mod prelude {
    macro_rules! entity_trait_get_value {
        () => {
            type Value = EntityValue;
            fn get_value(&self) -> &Self::Value {
                &self.value.0
            }
            fn get_cloned(&self) -> std::collections::HashMap<String, serde_json::Value> {
                self.value.0.get_cloned()
            }
        };
    }

    pub(crate) use entity_trait_get_value;

    pub use super::{EntityItemTrait, EntityTrait, EntityValueTrait};
}
