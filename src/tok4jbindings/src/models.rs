use jni::objects::{JClass, JObject, JObjectArray, JString};
use jni::sys::{jlong, jobject};
use jni::JNIEnv;
use jni_fn::jni_fn;
use tk::models::bpe::BPE;
use tokenizers as tk;

use tk::models::ModelWrapper;
use tk::Model;

use crate::error::{Context, Result};
use crate::java_classable::JavaClassable;

struct JModel {
    handle: jlong,
}

unsafe impl JavaClassable for JModel {
    const LOC: &'static str = "dev/gigapixel/tok4j/Model";
    const PATH: &'static str = "dev.gigapixel.tok4j.Model";
    type RustType = ModelWrapper;
    fn new_from_handle(handle: jlong) -> Self {
        JModel { handle }
    }

    fn handle(self) -> jlong {
        self.handle
    }
}

#[jni_fn("dev.gigapixel.tok4j.Model")]
pub fn dropByHandle<'local>(mut _env: JNIEnv<'local>, _class: JClass<'local>, handle: jlong) {
    unsafe { JModel::drop_by_handle(handle) };
}

#[jni_fn("dev.gigapixel.tok4j.Model")]
pub fn newModel<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    _str: JString<'local>,
) -> JObject<'local> {
    let res = unsafe {
        JModel::new_from_rust_type(&mut env, || -> ModelWrapper { BPE::default().into() })
    };
    match res {
        Ok(object) => object.1,
        Err(err) => {
            env.throw_new(
                "java/lang/Exception",
                &format!("Failed to instantiate BPE model: {err}"),
            )
            .expect("Failed to throw exception");
            JObject::null()
        }
    }
}

fn tokenize_internal<'local, 'borrow>(
    env: &'borrow mut JNIEnv<'local>,
    input: &'borrow JString<'local>,
    model: JModel,
) -> Result<JObjectArray<'local>> {
    model.use_shared(move |model| {
        let sequence: String = env
            .get_string(input)
            .context("Failed to get string from java string in tokenize".to_string())?
            .into();
        let result = model.tokenize(&sequence)?;
        let empty_str = env.new_string("")?;
        let str_class = env.get_object_class(&empty_str)?;

        let mut result_obj = env.new_object_array(result.len() as i32, str_class, empty_str)?;
        for (i, token) in result.into_iter().enumerate() {
            let new_string = env
                .new_string(token.value)
                .context("Failed to create new string".into())?;
            env.set_object_array_element(&mut result_obj, i as i32, new_string)
                .context("Failed to set array element".into())?;
        }
        Ok(result_obj)
    })
}

#[jni_fn("dev.gigapixel.tok4j.Model")]
pub fn tokenize<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
    sequence: JString<'local>,
) -> JObject<'local> {
    //JObjectArray<'local> {
    let model = JModel::new_from_handle(handle);
    match tokenize_internal(&mut env, &sequence, model) {
        Ok(object) => object.into(),
        Err(err) => {
            env.throw_new("java/lang/Exception", &format!("Failed to tokenize: {err}"))
                .expect("Failed to throw exception");
            JObject::null()
        }
    }
}

// Get the ID associated to a token
//
// Args:
//     token (:obj:`str`):
//         A token to convert to an ID
//
// Returns:
//     :obj:`int`: The ID associated to the token
// #[pyo3(text_signature = "(self, tokens)")]
// fn token_to_id(&self, token: &str) -> Option<u32> {
//     self.model.read().unwrap().token_to_id(token)
// }

// Get the token associated to an ID
//
// Args:
//     id (:obj:`int`):
//         An ID to convert to a token
//
// Returns:
//     :obj:`str`: The token associated to the ID
// #[pyo3(text_signature = "(self, id)")]
// fn id_to_token(&self, id: u32) -> Option<String> {
//     self.model.read().unwrap().id_to_token(id)
// }

// Save the current model
//
// Save the current model in the given folder, using the given prefix for the various
// files that will get created.
// Any file with the same name that already exists in this folder will be overwritten.
//
// Args:
//     folder (:obj:`str`):
//         The path to the target folder in which to save the various files
//
//     prefix (:obj:`str`, `optional`):
//         An optional prefix, used to prefix each file name
//
// Returns:
//     :obj:`List[str]`: The list of saved files
// #[pyo3(text_signature = "(self, folder, prefix)")]
// fn save<'a>(
//     &self,
//     py: Python<'_>,
//     folder: &str,
//     mut prefix: Option<&'a str>,
//     name: Option<&'a str>,
// ) -> PyResult<Vec<String>> {
//     if name.is_some() {
//         deprecation_warning(
//             py,
//             "0.10.0",
//             "Parameter `name` of Model.save has been renamed `prefix`",
//         )?;
//         if prefix.is_none() {
//             prefix = name;
//         }
//     }
//
//     let saved: PyResult<Vec<_>> =
//         ToPyResult(self.model.read().unwrap().save(Path::new(folder), prefix)).into();
//
//     Ok(saved?
//         .into_iter()
//         .map(|path| path.to_string_lossy().into_owned())
//         .collect())
// }

// Get the associated :class:`~tokenizers.trainers.Trainer`
//
// Retrieve the :class:`~tokenizers.trainers.Trainer` associated to this
// :class:`~tokenizers.models.Model`.
//
// Returns:
//     :class:`~tokenizers.trainers.Trainer`: The Trainer used to train this model
// #[pyo3(text_signature = "(self)")]
// fn get_trainer(&self, py: Python<'_>) -> PyResult<PyObject> {
//     PyTrainer::from(self.model.read().unwrap().get_trainer()).get_as_subtype(py)
// }
