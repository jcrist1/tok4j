use jni::objects::{JByteArray, JClass, JObject, JObjectArray, JString};
use jni::sys::jlong;
use jni::JNIEnv;
use jni_fn::jni_fn;
use tk::Tokenizer;
use tokenizers as tk;

use crate::error::{JError, Result};
use crate::java_classable::JavaClassable;

struct TkWrapper(Tokenizer);

unsafe impl JavaClassable for TkWrapper {
    const LOC: &'static str = "dev/gigapixel/tok4j/Tokenizer";

    const PATH: &'static str = "dev.gigapixel.tok4j.Tokenizer";
}

impl TkWrapper {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(TkWrapper(Tokenizer::from_bytes(bytes)?))
    }
}

pub fn tokenize_<'local>(
    env: &mut JNIEnv<'local>,
    handle: jlong,
    string: &JString<'local>,
) -> Result<JObjectArray<'local>> {
    let string = env.get_string(string)?;
    let string = string.to_str()?;
    let tokens = TkWrapper::use_shared(handle, |tokenizer| tokenizer.0.encode(string, true))?;
    let tokens = tokens.get_tokens();
    let empty_str = env.new_string("")?;
    let str_class = env.get_object_class(&empty_str)?;
    let mut return_array = env.new_object_array(tokens.len() as i32, &str_class, &empty_str)?;
    for (i, token) in tokens.iter().enumerate() {
        let j_token = env.new_string(token)?;
        env.set_object_array_element(&mut return_array, i as i32, j_token)?;
    }
    Ok(return_array)
}

pub fn from_bytes_<'local>(
    env: &mut JNIEnv<'local>,
    bytes: &JByteArray<'local>,
) -> Result<JObject<'local>> {
    let bytes: &[u8] = &env.convert_byte_array(bytes)?;
    let tokenizer = TkWrapper::from_bytes(bytes)?;
    unsafe { tokenizer.new_from_rust_type(env) }
}

#[jni_fn("dev.gigapixel.tok4j.Tokenizer")]
pub fn fromBytes<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    bytes: JByteArray<'local>,
) -> JObject<'local> {
    from_bytes_(&mut env, &bytes).j_throw(&mut env)
}

#[jni_fn("dev.gigapixel.tok4j.Tokenizer")]
pub fn tokenize<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
    bytes: JString<'local>,
) -> JObjectArray<'local> {
    tokenize_(&mut env, handle, &bytes).j_throw(&mut env)
}
