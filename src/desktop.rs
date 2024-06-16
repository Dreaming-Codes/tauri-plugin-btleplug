use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime, Manager};
use btleplug::platform::Manager as BtleManager;
use tauri::async_runtime::TokioRuntime;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Btleplug<R>> {
    Ok(Btleplug(app.clone()))
}

/// Access to the btleplug APIs.
pub struct Btleplug<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Btleplug<R> {
    pub fn get_manager(&self) -> crate::Result<BtleManager> {
        let state = self.0.app_handle().state::<crate::PluginState>().inner();

        state.manager.lock().unwrap().as_ref().ok_or(crate::Error::ManagerNotInitialized).cloned()
    }

    pub fn get_runtime(&self) -> crate::Result<&TokioRuntime> {
        crate::TOKIO_RUNTIME.get().as_ref().cloned().ok_or(crate::Error::RuntimeNotInitialized)
    }
}
