use indoc::indoc;

static CREATE_SUCCESS_MESSAGE: &str = "Got it. Here's your code word: ";

pub fn create_success(code: String) -> String {
    format!("{} {}", CREATE_SUCCESS_MESSAGE, code)
}

static UNKNOWN_ERROR_MESSAGE: &str =
    "Whoops! Something went wrong. Try again later.";

pub fn create_unknown_error() -> String {
    UNKNOWN_ERROR_MESSAGE.to_string()
}

pub fn find_unknown_error() -> String {
    UNKNOWN_ERROR_MESSAGE.to_string()
}

static CREATE_OVER_CAPACITY_ERROR_MESSAGE: &str =
    "Whoops! The code word database is full. Try again later.";

pub fn create_over_capacity() -> String {
    CREATE_OVER_CAPACITY_ERROR_MESSAGE.to_string()
}

static FIND_NOT_FOUND_ERROR_MESSAGE: &str =
    "Whoops! That code word doesn't exist.";

static CREATE_HOW_TO_MESSAGE: &str = indoc! {"
        Use verbalcode to create a code word for your message by sending us a text in this format:
        verbalcode <your message here>
    "};

static FIND_HOW_TO_MESSAGE: &str =
    "If someone gave you a code word, send us a text with just that.";

pub fn find_not_found() -> String {
    format!(
        "{}\n\n{}\n\n{}",
        FIND_NOT_FOUND_ERROR_MESSAGE,
        CREATE_HOW_TO_MESSAGE,
        FIND_HOW_TO_MESSAGE
    )
}

pub fn prompt_malformed() -> String {
    format!("{}\n\n{}", CREATE_HOW_TO_MESSAGE, FIND_HOW_TO_MESSAGE)
}

pub fn prompt_create_message_invalid(_reason: String) -> String {
    format!("{}\n\n{}", CREATE_HOW_TO_MESSAGE, FIND_HOW_TO_MESSAGE)
}

pub fn find_found(message: String) -> String {
    format!("{}\n\n{}", message, CREATE_HOW_TO_MESSAGE)
}
