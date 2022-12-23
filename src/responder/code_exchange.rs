use std::result;

pub(crate) enum WriteError {
    Invalid(String),
}

pub(crate) type WriteResult = result::Result<String, WriteError>;

pub(crate) fn write(message: String) -> WriteResult {
    todo!()
}

pub(crate) enum ReadError {
    NotFound,
}

pub(crate) type ReadResult = result::Result<String, ReadError>;

pub(crate) fn read(code: String) -> ReadResult {
    todo!()
}
