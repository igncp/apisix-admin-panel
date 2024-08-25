use std::collections::HashSet;

use crate::macros::derive_common_default;
use ts_rs::TS;

derive_common_default! {
#[derive(TS, Hash, Eq, PartialEq)]
#[ts(export)]
pub enum PluginEntities {
    #[default]
    Consumer,
    Route,
    Service,
    ConsumerGroup,
    Upstream,
}}

derive_common_default! {
#[derive(TS, Hash, Eq, PartialEq)]
#[ts(export)]
pub enum PluginPropertyType {
    #[default]
    String,
    Boolean,
    Number,
    Value,
    List(Box<PluginPropertyType>),
    Enum(Vec<String>),
}}

derive_common_default! {
#[derive(TS)]
#[ts(export)]
pub struct PluginOption {
    pub default_value: Option<String>,
    pub description: String,
    pub is_required: bool,
    pub name: String,
    pub property_type: PluginPropertyType,
}}

derive_common_default! {
#[derive(TS)]
#[ts(export)]
pub struct PluginDefinition {
    pub entities: HashSet<PluginEntities>,
    pub name: String,
    pub options: Vec<PluginOption>,
}}
