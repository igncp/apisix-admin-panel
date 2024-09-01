use crate::{
    apisix::base::{PropertyType, Required},
    macros::derive_common_default,
};
use std::collections::HashSet;
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
#[derive(TS)]
#[ts(export)]
pub struct PluginOption {
    pub default_value: Option<String>,
    pub description: String,
    pub is_required: Required,
    pub name: String,
    pub property_type: PropertyType,
}}

derive_common_default! {
#[derive(TS)]
#[ts(export)]
pub struct PluginDefinition {
    pub entities: HashSet<PluginEntities>,
    pub name: String,
    pub options: Vec<PluginOption>,
}}
