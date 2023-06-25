use serde::{Serialize, Deserialize};
use ts_rs::TS;
use strum_macros::Display;

pub const COMBATTEN_DOMAIN: &str = "combatten";

/// Dto used for combatten edit operation
#[derive(TS, Serialize, Deserialize)]
#[ts(export, rename_all="camelCase")]
#[ts(export_to = "../src/bindings/edit-name-dto.ts")]
#[serde(rename_all(deserialize="camelCase", serialize="camelCase"))]
pub struct EditNameDto {
    pub id: String,
    pub new_name: String
}

/// Actions related to combatten entities
#[derive(Serialize, Deserialize, Display)]
#[serde(rename_all(serialize="camelCase", deserialize="camelCase"), tag = "type", content = "payload")]
#[strum(serialize_all = "camelCase")]
pub enum CombattenAction {
    RenameCombatten(EditNameDto),
    CancelCombattenRename { id: String },
    CombattenRenamed(EditNameDto),
    CombattenRenameCanceled(EditNameDto),
    CombattenRenameError
}