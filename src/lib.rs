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

            #[cfg(target_os = "android")]
            {
                app.listen("tauri://btleplug/init", move |msg| {
                    let ctx = ndk_context::android_context();

                    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
                    let env = vm.attach_current_thread().unwrap();
                    //let context = unsafe { jni::objects::JObject::from_raw(ctx.context().cast()) };
                    btleplug::platform::init(&env).unwrap();
                    println!("btleplug initialized");
                });
            }

            Ok(())
        })
        .build()
}
