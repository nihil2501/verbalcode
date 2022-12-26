use std::result;

pub(crate) enum CreateError {
    Invalid(String),
}

pub(crate) type CreateResult = result::Result<String, CreateError>;

pub(crate) fn create(message: String) -> CreateResult {
    todo!()
}

pub(crate) enum FindError {
    NotFound,
}

pub(crate) type FindResult = result::Result<String, FindError>;

pub(crate) fn find(code: String) -> FindResult {
    todo!()
}
