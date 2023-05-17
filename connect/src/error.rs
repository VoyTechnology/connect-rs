use crate::code::Code;

#[derive(Debug)]
pub struct Error {
    code: Code,
    cause: Box<dyn std::error::Error>,
}

impl Error {
    pub fn new(code: Code, cause: impl std::error::Error + 'static) -> Self {
        Error {
            code,
            cause: Box::new(cause),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]: {}", self.code, self.cause)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.cause)
    }
}
