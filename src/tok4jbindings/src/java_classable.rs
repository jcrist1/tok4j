use std::{borrow::BorrowMut, pin::Pin};

use jni::{
    objects::{JObject, JValue},
    sys::jlong,
    JNIEnv,
};
use std::sync::RwLock;

use crate::error::{Context, Result};

/// # Safety
/// This trait receives a java long as a pointer. The java class has to ensure that
/// a cleaner is correctly configured to dispose of the data pointed to.
/// Additionally the only way for a cleaner to be configured assumes that the associate handle is
/// never changed until the instances is cleaned up by the GC.
pub(crate) unsafe trait JavaClassable
where
    Self: Sized,
{
    const LOC: &'static str;
    const PATH: &'static str;
    type RustType;

    fn new_from_handle(handle: jlong) -> Self;
    fn handle(self) -> jlong;

    unsafe fn new_from_rust_type<'local, F: Fn() -> Self::RustType>(
        env: &mut JNIEnv<'local>,
        f: F,
    ) -> Result<(Self, JObject<'local>)> {
        let class = format!("L{};", Self::LOC);
        let mut return_obj = env
            //.new_object("Ldev/gigapixel/tok4j/Model;", "()V", &[])
            .new_object(&class, "()V", &[])
            .context("Boop".into())?;

        //.context(format!("Failed to instantiate object {}", Self::PATH))?;
        // let mut return_obj = env
        //     .new_object(&class, "()", &[])
        //     .context(format!("Failed to instantiate object {}", Self::PATH))?;
        let self_instance = f();

        let boxed = Box::new(RwLock::new(self_instance));

        let handle = Box::into_raw(boxed) as jlong;

        env.set_field(&mut return_obj, "handle", "J", JValue::Long(handle))
            .context(format!(
                "Failed to set handle pointer for java object: {}",
                Self::PATH
            ))?;
        // env.set_field(&mut return_obj, "handle", "J", JValue::Long(handle))
        //     .context(format!(
        //         "Failed to set handle pointer for java object: {}",
        //         Self::PATH
        //     ))?;
        Ok((Self::new_from_handle(handle), return_obj))
    }

    unsafe fn rust_type(self) -> Box<RwLock<Self::RustType>> {
        unsafe { Box::from_raw(self.handle() as *mut RwLock<Self::RustType>) }
    }

    fn use_shared<T, F: FnOnce(&Self::RustType) -> T>(self, f: F) -> T {
        let rust_type = unsafe { self.rust_type() };
        let t = f(&rust_type.read().expect("Failed to readRwLock"));
        // garbage collector has to clean up
        std::mem::forget(rust_type);
        t
    }

    fn use_mut<T, F: FnOnce(&mut Self::RustType) -> T>(self, f: F) -> T {
        let rust_type = unsafe { self.rust_type() };
        let t = f(rust_type
            .write()
            .expect("Failed to lock RwLock")
            .borrow_mut());
        // garbage collector has to clean up
        std::mem::forget(rust_type);
        t
    }
    unsafe fn drop_by_handle(handle: jlong) {
        unsafe {
            let _ = Box::from_raw(handle as *mut RwLock<Self::RustType>);
        }
    }
}
