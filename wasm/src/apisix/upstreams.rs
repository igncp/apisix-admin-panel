use crate::macros::{derive_common, entity_fields_impl, entity_impl, entity_list_impl};
use apisix_admin_panel_core::apisix::upstreams::{GetUpstreamsResponse, Upstream, UpstreamEntity};

derive_common! {
pub struct WasmUpstream(UpstreamEntity);}

derive_common! {
pub struct WasmGetUpstreamsResponse(GetUpstreamsResponse);}

entity_impl! {WasmUpstream, Upstream}
entity_list_impl! {WasmGetUpstreamsResponse, WasmUpstream}
entity_fields_impl! {WasmUpstream, UpstreamEntity, WasmGetUpstreamsResponse, Upstream}
