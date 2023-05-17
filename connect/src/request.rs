/// Request is a wrapper around a generated request message. It provides
/// access to metadata like headers and the RPC specification, as well as
/// strongly-typed access to the message itself.
pub struct Request<T> {
    pub msg: T,
}

impl<T> Request<T> {
    pub fn new(message: T) -> Self {
        Self { msg: message }
    }
}
