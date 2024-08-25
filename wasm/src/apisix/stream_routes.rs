use crate::macros::{derive_common, entity_fields_impl, entity_impl, entity_list_impl};
use apisix_admin_panel_core::apisix::stream_routes::{
    GetStreamRoutesResponse, StreamRoute, StreamRouteEntity,
};

derive_common! {
pub struct WasmStreamRoute(StreamRouteEntity);}

derive_common! {
pub struct WasmGetStreamRoutesResponse(GetStreamRoutesResponse);}

entity_impl! {WasmStreamRoute, StreamRoute}
entity_list_impl! {WasmGetStreamRoutesResponse, WasmStreamRoute}
entity_fields_impl! {WasmStreamRoute, StreamRouteEntity, WasmGetStreamRoutesResponse, StreamRoute}
