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



#[cfg(target_os = "android")]
pub static JAVAVM: OnceCell<jni::JavaVM> = OnceCell::new();

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn JNI_OnLoad(vm: jni::JavaVM, res: *const std::os::raw::c_void) -> jni::sys::jint {
    let env = vm.get_env().unwrap();
    jni_utils::init(&env).unwrap();
    btleplug::platform::init(&env).unwrap();
    let _ = JAVAVM.set(vm);
    jni::JNIVersion::V6.into()
}


/// Access to the btleplug APIs.
pub struct Btleplug<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Btleplug<R> {}
