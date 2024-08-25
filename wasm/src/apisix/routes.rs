use crate::macros::{derive_common, entity_fields_impl, entity_impl, entity_list_impl};
use apisix_admin_panel_core::apisix::routes::{GetRoutesResponse, Route, RouteEntity};

derive_common! {
pub struct WasmRoute(RouteEntity);}

derive_common! {
pub struct WasmGetRoutesResponse(GetRoutesResponse);}

entity_impl! {WasmRoute, Route}
entity_list_impl! {WasmGetRoutesResponse, WasmRoute}
entity_fields_impl! {WasmRoute, RouteEntity, WasmGetRoutesResponse, Route}
