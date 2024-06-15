use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

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
mod permission;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the btleplug APIs.
pub trait BtleplugExt<R: Runtime> {
    fn btleplug(&self) -> &Btleplug<R>;
}

impl<R: Runtime, T: Manager<R>> crate::BtleplugExt<R> for T {
    fn btleplug(&self) -> &Btleplug<R> {
        self.state::<Btleplug<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("btleplug")
        .setup(|app, api| {
            #[cfg(mobile)]
            let btleplug = mobile::init(app, api)?;
            #[cfg(desktop)]
            let btleplug = desktop::init(app, api)?;
            app.manage(btleplug);

            Ok(())
        })
        .build()
}
