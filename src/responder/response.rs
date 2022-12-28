const CREATE_SUCCESS_MESSAGE: &str = "CODE_WRITE_SUCCESS_MESSAGE";
const CREATE_HOW_TO_MESSAGE: &str = "CREATE_HOW_TO_MESSAGE";
const CREATE_OVER_CAPACITY_ERROR_MESSAGE: &str =
    "CREATE_OVER_CAPACITY_ERROR_MESSAGE";
const FIND_NOT_FOUND_ERROR_MESSAGE: &str = "CODE_NOT_FOUND_ERROR_MESSAGE";
const FIND_HOW_TO_MESSAGE: &str = "FIND_HOW_TO_MESSAGE";
const UNKNOWN_ERROR_MESSAGE: &str = "UNKNOWN_ERROR_MESSAGE";

pub fn create_valid(code: String) -> String {
    format!("{}\n{}", CREATE_SUCCESS_MESSAGE, code)
}

pub fn create_unknown_error() -> String {
    UNKNOWN_ERROR_MESSAGE.to_string()
}

pub fn create_over_capacity() -> String {
    CREATE_OVER_CAPACITY_ERROR_MESSAGE.to_string()
}

pub fn find_found(message: String) -> String {
    format!("{}\n\n{}", message, CREATE_HOW_TO_MESSAGE)
}

pub fn find_not_found() -> String {
    format!(
        "{}\n\n{}\n\n{}",
        FIND_NOT_FOUND_ERROR_MESSAGE,
        CREATE_HOW_TO_MESSAGE,
        FIND_HOW_TO_MESSAGE
    )
}

pub fn find_unknown_error() -> String {
    UNKNOWN_ERROR_MESSAGE.to_string()
}

pub fn prompt_malformed() -> String {
    format!("{}\n\n{}", CREATE_HOW_TO_MESSAGE, FIND_HOW_TO_MESSAGE)
}

pub fn prompt_create_message_invalid(_reason: String) -> String {
    format!("{}\n\n{}", CREATE_HOW_TO_MESSAGE, FIND_HOW_TO_MESSAGE)
}
