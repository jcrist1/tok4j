use jni::JNIEnv;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Jni Error {0}")]
    Jni(#[from] jni::errors::Error),
    #[error("Tokenizer error: {0}")]
    Tokenizer(#[from] tokenizers::Error),
    #[error("Error trying to read JavaString as utf8: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("{context_message}, caused by {err}")]
    Contextual {
        context_message: String,
        err: Box<Error>,
    },
}

impl Error {
    fn context(self, message: String) -> Error {
        match self {
            Error::Contextual {
                context_message,
                err,
            } => {
                let context_message = format!("{message}\n{context_message}");
                Error::Contextual {
                    context_message,
                    err,
                }
            }
            err => Error::Contextual {
                context_message: message,
                err: Box::new(err),
            },
        }
    }
}

pub(crate) trait JError<T> {
    fn j_throw(self, env: &mut JNIEnv<'_>) -> T;
}

impl<T: Default> JError<T> for Result<T> {
    fn j_throw(self, env: &mut JNIEnv<'_>) -> T {
        match self {
            Err(err) => {
                env.throw_new("java/lang/Exception", format!("{err}"))
                    .unwrap_or_else(|err_2| {
                        panic!("Failed to throw exception: {err_2}. Trying to throw {err}")
                    });
                T::default()
            }
            Ok(inner) => inner,
        }
    }
}

pub(crate) trait Context<T> {
    fn context(self, message: String) -> Result<T>;
}

impl<T, E> Context<T> for std::result::Result<T, E>
where
    E: Into<Error>,
{
    fn context(self, message: String) -> Result<T> {
        self.map_err(Into::into).map_err(|err| err.context(message))
    }
}
