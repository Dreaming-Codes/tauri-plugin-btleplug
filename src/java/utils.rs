#![cfg(target_os = "android")]

use std::cell::RefCell;
use jni::{AttachGuard, JNIEnv};
use jni::objects::GlobalRef;
use tokio::sync::OnceCell;
use std::sync::atomic::{AtomicUsize, Ordering};

static CLASS_LOADER: OnceCell<GlobalRef> = OnceCell::const_new();
pub(crate) static JAVAVM: OnceCell<jni::JavaVM> = OnceCell::const_new();
pub(crate) static RUNTIME: OnceCell<tokio::runtime::Runtime> = OnceCell::const_new();

pub(crate) fn setup_class_loader(env: &JNIEnv) {
    let thread = env
        .call_static_method(
            "java/lang/Thread",
            "currentThread",
            "()Ljava/lang/Thread;",
            &[],
        )
        .expect("Failed to get current java thread")
        .l()
        .expect("Failed to get current java thread object");
    let class_loader = env
        .call_method(
            thread,
            "getContextClassLoader",
            "()Ljava/lang/ClassLoader;",
            &[],
        )
        .expect("Failed to get class loader")
        .l()
        .expect("Failed to get class loader object");

    let _ = CLASS_LOADER.set(env.new_global_ref(class_loader).expect("Failed to create global ref"));
}

pub(crate) fn load_btleplug_context() {
    let vm = JAVAVM.get().expect("JavaVM not set");
    let env = vm.attach_current_thread().expect("Failed to attach to JVM");

    let thread = env
        .call_static_method(
            "java/lang/Thread",
            "currentThread",
            "()Ljava/lang/Thread;",
            &[],
        )
        .expect("Failed to get current java thread")
        .l()
        .expect("Failed to get current java thread object");
    env.call_method(
        thread,
        "setContextClassLoader",
        "(Ljava/lang/ClassLoader;)V",
        &[CLASS_LOADER.get().unwrap().as_obj().into()],
    )
        .expect("Failed to set class loader context");
}

std::thread_local! {
    static JNI_ENV: RefCell<Option<AttachGuard<'static>>> = RefCell::new(None);
}

pub(crate) fn create_runtime() {
    let vm = JAVAVM.get().expect("JavaVM not set");
    let env = vm.attach_current_thread().expect("Failed to attach to JVM");

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name_fn(|| {
            static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
            let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
            format!("tauri-plugin-btleplug-{}", id)
        })
        .on_thread_stop(|| {
            JNI_ENV.with(|f| *f.borrow_mut() = None);
        })
        .on_thread_start(move || {
            let vm = JAVAVM.get().expect("JavaVM not set");
            let env = vm.attach_current_thread().expect("Failed to attach to JVM");

            load_btleplug_context();

            JNI_ENV.with(|f| *f.borrow_mut() = Some(env));
        })
        .build()
        .expect("Failed to create runtime");
    
    RUNTIME.set(runtime).expect("Failed to set runtime");
}
