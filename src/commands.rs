use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::BroadcastExt;

#[command]
pub(crate) async fn register<R: Runtime>(
    app: AppHandle<R>,
    action: String,
    handler: tauri::ipc::Channel<BroadcastEvent>,
) -> Result<()> {
    app.broadcast().register(RegisterPayload { action, handler })
}

#[command]
pub(crate) async fn unregister<R: Runtime>(
    app: AppHandle<R>,
    payload: UnregisterRequest,
) -> Result<()> {
    app.broadcast().unregister(payload)
}
