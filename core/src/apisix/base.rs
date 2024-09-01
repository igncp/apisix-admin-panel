use crate::macros::derive_common_default;
use ts_rs::TS;

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
    JSON,
    Enum(Vec<String>),
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
