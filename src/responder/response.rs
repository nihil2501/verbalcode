const CREATE_SUCCESS_MESSAGE: &str = "CODE_WRITE_SUCCESS_MESSAGE";
const CREATE_INVALID_ERROR_MESSAGE: &str = "CODE_INVALID_ERROR_MESSAGE";
const CREATE_HOW_TO_MESSAGE: &str = "CREATE_HOW_TO_MESSAGE";
const FIND_NOT_FOUND_ERROR_MESSAGE: &str = "CODE_NOT_FOUND_ERROR_MESSAGE";
const FIND_HOW_TO_MESSAGE: &str = "FIND_HOW_TO_MESSAGE";

pub(crate) fn create_valid(code: String) -> String {
    format!("{}\n{}", CREATE_SUCCESS_MESSAGE, code)
}

pub(crate) fn create_invalid(reason: String) -> String {
    format!("{}\n{}", CREATE_INVALID_ERROR_MESSAGE, reason)
}

pub(crate) fn find_found(message: String) -> String {
    format!("{}\n\n{}", message, CREATE_HOW_TO_MESSAGE)
}

pub(crate) fn find_not_found() -> String {
    format!(
        "{}\n\n{}\n\n{}",
        FIND_NOT_FOUND_ERROR_MESSAGE,
        CREATE_HOW_TO_MESSAGE,
        FIND_HOW_TO_MESSAGE
    )
}

pub(crate) fn prompt_malformed() -> String {
    format!("{}\n\n{}", CREATE_HOW_TO_MESSAGE, FIND_HOW_TO_MESSAGE)
}

pub(crate) fn prompt_create_message_missing() -> String {
    format!("{}\n\n{}", CREATE_HOW_TO_MESSAGE, FIND_HOW_TO_MESSAGE)
}
