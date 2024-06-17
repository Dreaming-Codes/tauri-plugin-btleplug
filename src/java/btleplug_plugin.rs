#![cfg(target_os = "android")]

use jni::JNIEnv;
use jni::objects::{JClass};
use crate::java::utils::{setup_runtime, JAVAVM, setup_class_loader};

#[no_mangle]
pub extern "system" fn Java_com_plugin_btleplug_BtleplugPlugin_init(
    env: JNIEnv,
    _class: JClass
) {
    btleplug::platform::init(&env).expect("Failed to initialize btleplug platform");

    setup_class_loader(&env).expect("Failed to set class loader");

    if let Err(_) = JAVAVM.set(env.get_java_vm().expect("Failed to get JavaVM")) {
        panic!("Failed to set JavaVM for btleplug plugin");
    }

    setup_runtime().expect("Failed to setup tokio runtime");
}
