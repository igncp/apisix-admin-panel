use crate::macros::{derive_common, entity_fields_impl, entity_impl, entity_list_impl};
use apisix_admin_panel_core::apisix::consumers::{Consumer, ConsumerEntity, GetConsumersResponse};

derive_common! {
pub struct WasmConsumer(ConsumerEntity);}

derive_common! {
pub struct WasmGetConsumersResponse(GetConsumersResponse);}

entity_impl! {WasmConsumer, Consumer}
entity_list_impl! {WasmGetConsumersResponse, WasmConsumer}
entity_fields_impl! {WasmConsumer, ConsumerEntity, WasmGetConsumersResponse, Consumer}
