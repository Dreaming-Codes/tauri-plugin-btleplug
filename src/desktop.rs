use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Btleplug<R>> {
    Ok(Btleplug(app.clone()))
}

/// Access to the btleplug APIs.
pub struct Btleplug<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Btleplug<R> {
    pub fn btleplug_context_spawn<F>(&self, future: F) -> tokio::task::JoinHandle<F::Output>
    where
        F: std::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        tokio::spawn(future)
    }
    
}
