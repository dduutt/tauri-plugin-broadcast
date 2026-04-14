use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterPayload {
  pub action: String,
  pub handler: tauri::ipc::Channel<BroadcastEvent>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnregisterRequest {
  pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastEvent {
  pub action: String,
  pub extras: serde_json::Value,
}
