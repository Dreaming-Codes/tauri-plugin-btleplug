use serde::de::DeserializeOwned;
use tauri::{plugin::{PluginApi, PluginHandle}, AppHandle, Runtime, Manager};
use crate::permission::{PermissionResponse, RequestPermission};
use btleplug::platform::Manager as BtleManager;
use tauri::async_runtime::TokioRuntime;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "com.plugin.btleplug";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_btleplug);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Btleplug<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "BtleplugPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_btleplug)?;

    Ok(Btleplug(handle))
}


/// Access to the btleplug APIs.
pub struct Btleplug<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Btleplug<R> {
    pub fn check_permissions(&self) -> crate::Result<PermissionResponse> {
        self.0
            .run_mobile_plugin::<PermissionResponse>("checkPermissions", ())
            .map_err(Into::into)
    }

    pub fn request_permissions(&self, permissions: RequestPermission) -> crate::Result<PermissionResponse> {
        self.0
            .run_mobile_plugin::<PermissionResponse>("requestPermissions", permissions)
            .map_err(Into::into)
    }

    pub fn get_manager(&self) -> crate::Result<BtleManager> {
        let state = self.0.app().state::<crate::PluginState>().inner();

        state.manager.lock().unwrap().as_ref().ok_or(crate::Error::ManagerNotInitialized).cloned()
    }

    pub fn get_runtime(&self) -> crate::Result<&TokioRuntime> {
        crate::TOKIO_RUNTIME.get().as_ref().cloned().ok_or(crate::Error::RuntimeNotInitialized)
    }
}
