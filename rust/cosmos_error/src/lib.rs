use arrayvec::ArrayString;

pub type Result<T, E = AnyError> = core::result::Result<T, Error<E>>;

#[non_exhaustive]
pub enum Error<E> {
    OutOfGas,
    AccountNotFound,
    MessageHandlerNotFound,
    HandlerError(E),
}

pub struct AnyError {
    pub code: u32,
    pub handler_id: ArrayString<64>,
    pub message: ArrayString<256>,
}
