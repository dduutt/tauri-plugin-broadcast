use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_broadcast);

pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<Broadcast<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("com.plugin.broadcast", "BroadcastPlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_broadcast)?;
  Ok(Broadcast(handle))
}

pub struct Broadcast<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Broadcast<R> {
  pub fn register(&self, payload: RegisterPayload) -> crate::Result<()> {
    self.0.run_mobile_plugin("register", payload).map_err(Into::into)
  }

  pub fn unregister(&self, payload: UnregisterRequest) -> crate::Result<()> {
    self.0.run_mobile_plugin("unregister", payload).map_err(Into::into)
  }
}
