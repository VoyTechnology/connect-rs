pub struct Response<T> {
    pub msg: T,
}

impl<T> Response<T> {
    pub fn new(message: T) -> Self {
        Self { msg: message }
    }
}
