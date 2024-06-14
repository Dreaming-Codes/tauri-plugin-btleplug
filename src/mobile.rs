use serde::de::DeserializeOwned;
use once_cell::sync::OnceCell;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

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

impl<R: Runtime> Btleplug<R> {}
