use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<Broadcast<R>> {
  Ok(Broadcast(app.clone()))
}

/// Access to the broadcast APIs.
pub struct Broadcast<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Broadcast<R> {
  pub fn register(&self, _payload: RegisterPayload) -> crate::Result<()> {
    Ok(())
  }

  pub fn unregister(&self, _payload: UnregisterRequest) -> crate::Result<()> {
    Ok(())
  }
}
