pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Jni Error {0}")]
    Jni(#[from] jni::errors::Error),
    #[error("Tokenizer error: {0}")]
    Tokenizer(#[from] tokenizers::Error),
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
