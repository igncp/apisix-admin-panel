use apisix_admin_panel_core::apisix::secrets::{GetSecretsResponse, Secret, SecretEntity};

use crate::macros::{derive_common, entity_fields_impl, entity_impl, entity_list_impl};

derive_common! {
pub struct WasmSecret(SecretEntity);}

derive_common! {
pub struct WasmGetSecretsResponse(GetSecretsResponse);}

entity_impl! {WasmSecret, Secret}
entity_list_impl! {WasmGetSecretsResponse, WasmSecret}
entity_fields_impl! {WasmSecret, SecretEntity, WasmGetSecretsResponse, Secret}
