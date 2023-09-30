use jni::{
    objects::{JClass, JObject, JObjectArray, JString, JValue},
    sys::jlong,
    JNIEnv,
};
use jni_fn::jni_fn;
use tokenizers::Tokenizer;

pub(crate) mod error;
mod java_classable;
mod models;

const TOKENIZER_BYTES: &[u8] = include_bytes!("../tokenizer.json");

struct JTokenizer;

trait JClassable {
    const LOC: &'static str;
    const PATH: &'static str;
    type RustType;
}
impl JClassable for JTokenizer {
    const LOC: &'static str = "dev/gigapixel/tok4j/Tokenizer";
    const PATH: &'static str = "dev.gigapixel.tok4j.Tokenizer";
    type RustType = Tokenizer;
}

//#[jni_fn("dev.gigapixel.tok4j.Tokenizer")]
#[jni_fn("dev.gigapixel.tok4j.Tokenizer")]
pub fn loadStatic<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    _input: JString<'local>,
) -> JObject<'local> {
    let tokenizer = Tokenizer::from_bytes(TOKENIZER_BYTES).expect("Failed to create tokenizer");
    let boxed = Box::new(tokenizer);

    let handle = Box::into_raw(boxed) as jlong;
    let mut j_tokenizer = env
        .new_object("Ldev/gigapixel/tok4j/Tokenizer;", "()V", &[])
        .expect("Failed to create model");
    env.set_field(&mut j_tokenizer, "handle", "J", JValue::Long(handle))
        .unwrap();
    j_tokenizer
}

unsafe fn tokenizer_from_java<'borrowed, 'local>(
    env: &'borrowed mut JNIEnv<'local>,
    class: &'borrowed JClass<'local>,
    handle: jlong,
) -> Box<Tokenizer> {
    // let handle = env
    //     .get_field(class, "handle", "J")
    //     .expect("Failed to get handle");
    //
    Box::from_raw(handle as *mut Tokenizer)
}

#[jni_fn("dev.gigapixel.tok4j.Tokenizer")]
pub fn dropByHandle<'borrowed, 'local>(
    _env: &'borrowed mut JNIEnv<'local>,
    _class: &'borrowed JClass<'local>,
    handle: jlong,
) {
    // let handle = env
    //     .get_field(class, "handle", "J")
    //     .expect("Failed to get handle");
    {
        unsafe {
            let _to_drop = Box::from_raw(handle as *mut Tokenizer);
        }
    }
}

#[jni_fn("dev.gigapixel.tok4j.Tokenizer")]
pub fn tokenizeFromHandle<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    handle: jlong,
    input: JString<'local>,
) -> JObjectArray<'local> {
    let tokenizer = unsafe { tokenizer_from_java(&mut env, &class, handle) };

    let input: String = env
        .get_string(&input)
        .expect("Failed to get string from input")
        .into();
    let tokenized = tokenizer
        .encode(input.as_str(), false)
        .expect("Failed to tokenize");
    let tokens = tokenized.get_tokens();

    let tokens = tokens
        .iter()
        .map(|token| {
            env.new_string(token.as_str())
                .expect("Failed to create java string from token")
        })
        .collect::<Vec<_>>();

    let empty_string = env.new_string("").expect("Failed to create empty string");
    let string_class = env
        .get_object_class(&empty_string)
        .expect("Failed to get string class");
    let mut return_arr = env
        .new_object_array(tokens.len() as i32, string_class, empty_string)
        .expect("Failed to instantiate return array");
    for (i, token) in tokens.into_iter().enumerate() {
        env.set_object_array_element(&mut return_arr, i as i32, token)
            .unwrap_or_else(|_| panic!("Failed to set index {i} of return array"))
    }
    return_arr
}

#[jni_fn("dev.gigapixel.tok4j.Tokenizer")]
pub fn tokenize<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    input: JString<'local>,
) -> JObjectArray<'local> {
    let tokenizer = Tokenizer::from_bytes(TOKENIZER_BYTES).expect("Failed to load tokenizer");
    let input: String = env
        .get_string(&input)
        .expect("Failed to get string from input")
        .into();
    let tokenized = tokenizer
        .encode(input.as_str(), false)
        .expect("Failed to tokenize");
    let tokens = tokenized.get_tokens();

    let tokens = tokens
        .iter()
        .map(|token| {
            env.new_string(token.as_str())
                .expect("Failed to create java string from token")
        })
        .collect::<Vec<_>>();

    let empty_string = env.new_string("").expect("Failed to create empty string");
    let string_class = env
        .get_object_class(&empty_string)
        .expect("Failed to get string class");
    let mut return_arr = env
        .new_object_array(tokens.len() as i32, string_class, empty_string)
        .expect("Failed to instantiate return array");
    for (i, token) in tokens.into_iter().enumerate() {
        env.set_object_array_element(&mut return_arr, i as i32, token)
            .unwrap_or_else(|_| panic!("Failed to set index {i} of return array"))
    }
    return_arr
}
