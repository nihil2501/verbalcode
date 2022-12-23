use regex::Regex;
use std::result;

mod code_exchange;

const CREATE_SUCCESS_MESSAGE: &str = "CODE_WRITE_SUCCESS_MESSAGE";
const CREATE_INVALID_ERROR_MESSAGE: &str = "CODE_INVALID_ERROR_MESSAGE";
const CREATE_HOW_TO_MESSAGE: &str = "CREATE_HOW_TO_MESSAGE";
const READ_NOT_FOUND_ERROR_MESSAGE: &str = "CODE_NOT_FOUND_ERROR_MESSAGE";
const READ_HOW_TO_MESSAGE: &str = "READ_HOW_TO_MESSAGE";

pub fn handle(prompt: String, prompter: String) -> String {
    // For now we just log prompter (but not in wasmcloud because of async
    // restriction to their logging crate).
    println!("prompt: {}, prompter: {}", prompt, prompter);

    // Prompt is either to create a code or read a code.
    // Prompt can either parse successfully as one of those actions or not.
    // [Labels for test scenarios collocated here.]
    match Action::parse(prompt) {
        // When prompt does parse correctly it is for one of these actions.
        Ok(action) => match action {
            // Prompt indicates that a code should be created for some message.
            Action::Create(message) => match write(message) {
                // Create is valid, yielding back a code corresponding to the
                // message.
                // test: `create_valid`
                Ok(code) => format!("{}\n{}", CREATE_SUCCESS_MESSAGE, code),

                // Message is invalid for some reason.
                // test: `create_invalid`
                Err(error) => match error {
                    code_exchange::WriteError::Invalid(reason) => {
                        format!("{}\n{}", CREATE_INVALID_ERROR_MESSAGE, reason)
                    }
                },
            },

            // Prompt indicates that a code should be read.
            Action::Read(code) => match read(code) {
                // Code exists in the exchange, yielding back the corresponding
                // message.
                // test: `read_found`
                Ok(message) => {
                    format!("{}\n\n{}", message, CREATE_HOW_TO_MESSAGE)
                }

                // Code doesn't exist in the exchange.
                // test: `read_not_found`
                Err(error) => match error {
                    code_exchange::ReadError::NotFound => {
                        format!(
                            "{}\n\n{}\n\n{}",
                            READ_NOT_FOUND_ERROR_MESSAGE,
                            CREATE_HOW_TO_MESSAGE,
                            READ_HOW_TO_MESSAGE
                        )
                    }
                },
            },
        },

        // When prompt doesn't parse correctly it does so in one of these ways.
        Err(error) => match error {
            // Prompt is so malformed it fails to indicate any action.
            // test: `prompt_malformed`
            PromptParseError::MalformedAction => {
                format!("{}\n\n{}", CREATE_HOW_TO_MESSAGE, READ_HOW_TO_MESSAGE)
            }

            // Prompt indicates a create but any kind of message argument is
            // absent.
            // test: `prompt_create_message_missing`
            PromptParseError::MessageMissing => {
                format!("{}\n\n{}", CREATE_HOW_TO_MESSAGE, READ_HOW_TO_MESSAGE)
            }
        },
    }
}

enum Action {
    Create(String),
    Read(String),
}

enum PromptParseError {
    MalformedAction,
    MessageMissing,
}

type PromptParseResult = result::Result<Action, PromptParseError>;
const PROMPT_REGEX: &str = r"\A([a-zA-Z]+)(\s+.*)?\z";

impl Action {
    fn parse(prompt: String) -> PromptParseResult {
        let exp = Regex::new(PROMPT_REGEX).unwrap();
        let mut matches = exp.captures_iter(prompt.trim());

        match matches.next() {
            Some(captures) => {
                let code = captures[1].to_lowercase();

                match code.as_str() {
                    "verbalcode" => {
                        let message =
                            captures.get(2).map_or("", |c| c.as_str().trim());

                        match message {
                            "" => Err(PromptParseError::MessageMissing),
                            _ => Ok(Action::Create(message.to_string())),
                        }
                    }
                    _ => Ok(Action::Read(code)),
                }
            }
            None => Err(PromptParseError::MalformedAction),
        }
    }
}

#[cfg(not(test))]
fn read(code: String) -> code_exchange::ReadResult {
    code_exchange::read(code)
}

#[cfg(not(test))]
fn write(message: String) -> code_exchange::WriteResult {
    code_exchange::write(message)
}

#[cfg(test)]
fn read(code: String) -> code_exchange::ReadResult {
    match code.as_str() {
        "foundcode" => Ok("found message".to_string()),
        "notfoundcode" => Err(code_exchange::ReadError::NotFound),
        _ => panic!(),
    }
}

#[cfg(test)]
fn write(message: String) -> code_exchange::WriteResult {
    match message.as_str() {
        "valid message" => Ok("validcode".to_string()),
        "invalid message" => Err(code_exchange::WriteError::Invalid(
            "invalid reason".to_string(),
        )),
        _ => panic!(),
    }
}

#[cfg(test)]
mod test {
    use crate::responder::*;

    #[test]
    fn create_valid() {
        let response = handle(
            "verbalcode valid message".to_string(),
            "prompter".to_string(),
        );

        assert_eq!(
            response,
            format!("{}\n{}", CREATE_SUCCESS_MESSAGE, "validcode")
        )
    }

    #[test]
    fn create_invalid() {
        let response = handle(
            "verbalcode invalid message".to_string(),
            "prompter".to_string(),
        );

        assert_eq!(
            response,
            format!("{}\n{}", CREATE_INVALID_ERROR_MESSAGE, "invalid reason")
        )
    }

    #[test]
    fn read_found() {
        let response = handle("foundcode".to_string(), "prompter".to_string());

        assert_eq!(
            response,
            format!("{}\n\n{}", "found message", CREATE_HOW_TO_MESSAGE)
        )
    }

    #[test]
    fn read_not_found() {
        let response =
            handle("notfoundcode".to_string(), "prompter".to_string());

        assert_eq!(
            response,
            format!(
                "{}\n\n{}\n\n{}",
                READ_NOT_FOUND_ERROR_MESSAGE,
                CREATE_HOW_TO_MESSAGE,
                READ_HOW_TO_MESSAGE
            )
        )
    }

    #[test]
    fn prompt_malformed() {
        let response =
            handle("verbalcode!".to_string(), "prompter".to_string());

        assert_eq!(
            response,
            format!("{}\n\n{}", CREATE_HOW_TO_MESSAGE, READ_HOW_TO_MESSAGE)
        );
    }

    #[test]
    fn prompt_create_message_missing() {
        let response =
            handle("verbalcode ".to_string(), "prompter".to_string());

        assert_eq!(
            response,
            format!("{}\n\n{}", CREATE_HOW_TO_MESSAGE, READ_HOW_TO_MESSAGE)
        )
    }
}
