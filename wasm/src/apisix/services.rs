use crate::macros::{derive_common, entity_fields_impl, entity_impl, entity_list_impl};
use apisix_admin_panel_core::apisix::services::{GetServicesResponse, Service, ServiceEntity};

derive_common! {
pub struct WasmService(ServiceEntity);}

derive_common! {
pub struct WasmGetServicesResponse(GetServicesResponse);}

entity_impl! {WasmService, Service}
entity_list_impl! {WasmGetServicesResponse, WasmService}
entity_fields_impl! {WasmService, ServiceEntity, WasmGetServicesResponse, Service}
