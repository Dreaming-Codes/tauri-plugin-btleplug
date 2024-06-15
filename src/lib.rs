use std::sync::{Arc, Mutex};
use btleplug::platform::Manager as BtleManager;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};
use tauri::async_runtime::spawn;

// Re-export the btleplug crate.
pub use btleplug;

#[cfg(desktop)]
use desktop::Btleplug;
pub use error::{Error, Result};
#[cfg(mobile)]
use mobile::Btleplug;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod error;
mod java;
pub mod permission;

#[derive(Default)]
pub(crate) struct PluginState {
    pub manager: Arc<Mutex<Option<BtleManager>>>
}

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the btleplug APIs.
pub trait BtleplugExt<R: Runtime> {
    fn btleplug(&self) -> &Btleplug<R>;
}

impl<R: Runtime, T: Manager<R>> crate::BtleplugExt<R> for T {
    fn btleplug(&self) -> &Btleplug<R> {
        self.state::<Btleplug<R>>().inner()
    }
}

#[cfg(mobile)]
lazy_static::lazy_static! {
    pub static ref INIT_SENDER: Mutex<Option<tokio::sync::oneshot::Sender<()>>> = Mutex::new(None);
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("btleplug")
        .setup(|app, api| {
            let state = PluginState::default();

            #[cfg(mobile)]
            {
                let (sender, mut receiver) = tokio::sync::oneshot::channel::<()>();
                *INIT_SENDER.lock().unwrap() = Some(sender);

                spawn({
                    let manager_handle = state.manager.clone();
                    async move {
                        receiver.await.expect("Failed to receive initialization signal");
                        let manager = BtleManager::new().await.expect("Failed to initialize btleplug manager");
                        manager_handle.lock().unwrap().replace(manager);
                    }
                });
            }

            #[cfg(desktop)]
            {
                let manager_state = state.manager.clone();
                spawn(async move {
                    let manager = BtleManager::new().await.expect("Failed to initialize btleplug manager");
                    manager_state.lock().unwrap().replace(manager);
                });
            }
            app.manage(state);

            #[cfg(mobile)]
            let btleplug = mobile::init(app, api)?;
            #[cfg(desktop)]
            let btleplug = desktop::init(app, api)?;

            app.manage(btleplug);
            Ok(())
        })
        .build()
}
