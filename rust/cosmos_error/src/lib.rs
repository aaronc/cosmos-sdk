use cosmos_message_api::SystemErrorCode;

pub enum Error<T> {
    SystemError(SystemErrorCode),
    DecodingError,
    HandlerError(T),
}
