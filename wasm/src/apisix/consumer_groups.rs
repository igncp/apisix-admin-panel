use crate::macros::{derive_common, entity_fields_impl, entity_impl, entity_list_impl};
use apisix_admin_panel_core::apisix::consumer_groups::{
    ConsumerGroup, ConsumerGroupEntity, GetConsumerGroupsResponse,
};

derive_common! {
pub struct WasmConsumerGroup(ConsumerGroupEntity);}

derive_common! {
pub struct WasmGetConsumerGroupsResponse(GetConsumerGroupsResponse);}

entity_impl! {WasmConsumerGroup, ConsumerGroup}
entity_list_impl! {WasmGetConsumerGroupsResponse, WasmConsumerGroup}
entity_fields_impl! {WasmConsumerGroup, ConsumerGroupEntity, WasmGetConsumerGroupsResponse, ConsumerGroup}
