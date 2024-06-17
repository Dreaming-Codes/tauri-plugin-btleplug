#![cfg(target_os = "android")]

use jni::JNIEnv;
use jni::objects::{JClass};

#[no_mangle]
pub extern "system" fn Java_com_plugin_btleplug_BtleplugPlugin_init(
    mut env: JNIEnv,
    _class: JClass
) {
    btleplug::platform::init(&env).unwrap();
}
