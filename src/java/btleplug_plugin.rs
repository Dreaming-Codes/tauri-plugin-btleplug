#![cfg(target_os = "android")]

use jni::JNIEnv;
use jni::objects::{JClass};

#[no_mangle]
pub extern "system" fn Java_com_plugin_btleplug_BtleplugPlugin_init(
    mut env: JNIEnv,
    _class: JClass
) {
    btleplug::platform::init(&env).unwrap();

    crate::INIT_SENDER.lock().expect("Failed to lock INIT_SENDER").take().expect("INIT_SENDER is None").send(()).expect("Failed to send initialization signal")
}
