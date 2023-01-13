use indoc::indoc;
use serde::Serialize;
use tinytemplate::TinyTemplate;

static CREATE_SUCCESS_MESSAGE_TEMPLATE: &str = indoc! {"
    Got it. Here's your code word: {code}
    It will expire in 24 hours.
"};

#[derive(Serialize)]
struct CreateSuccessContext {
    code: String,
}

pub fn create_success(code: String) -> String {
    let mut tt = TinyTemplate::new();
    tt.add_template("success", CREATE_SUCCESS_MESSAGE_TEMPLATE)
        .unwrap();
    tt.render("success", &CreateSuccessContext { code })
        .unwrap()
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

pub fn create_over_capacity_error() -> String {
    CREATE_OVER_CAPACITY_ERROR_MESSAGE.to_string()
}

static FIND_NOT_FOUND_ERROR_MESSAGE: &str =
    "Whoops! That code word doesn't exist.";

static CREATE_HOW_TO_MESSAGE: &str = indoc! {"
    Use partyskunk to create a code word for your message by sending us a text in this format:
    partyskunk <your message here>

    Code words expire after 24 hours.
"};

static FIND_HOW_TO_MESSAGE: &str =
    "If someone gave you a code word, send us a text with just that code word.";

pub fn find_not_found_error() -> String {
    format!(
        "{}\n\n{}\n\n{}",
        FIND_NOT_FOUND_ERROR_MESSAGE,
        CREATE_HOW_TO_MESSAGE,
        FIND_HOW_TO_MESSAGE
    )
}

pub fn prompt_malformed_error() -> String {
    format!("{}\n\n{}", CREATE_HOW_TO_MESSAGE, FIND_HOW_TO_MESSAGE)
}

pub fn prompt_create_message_invalid_error(_reason: String) -> String {
    format!("{}\n\n{}", CREATE_HOW_TO_MESSAGE, FIND_HOW_TO_MESSAGE)
}

pub fn find_success(message: String) -> String {
    format!("{}\n\n{}", message, CREATE_HOW_TO_MESSAGE)
}
