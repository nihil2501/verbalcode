use regex::Regex;
use std::result;

pub enum Action {
    Create(String),
    Read(String),
}

pub enum PromptParseError {
    MalformedAction,
    MessageInvalid(String),
}

type PromptParseResult = result::Result<Action, PromptParseError>;
const CREATE_PROMPT_WORD: &str = "partyskunk";
const MESSAGE_CHARACTER_LIMIT: usize = 140;
const MESSAGE_INVALID_REASON_MESSAGE: &str =
    "Message must not be empty and must be less than 140 characters.";

pub fn parse(prompt: String) -> PromptParseResult {
    let regex = Regex::new(r"\s+").unwrap();
    let mut split = regex.splitn(prompt.trim(), 2);

    match split.next() {
        Some(code) => match code.to_lowercase().as_str() {
            CREATE_PROMPT_WORD => match split.next() {
                Some(message) => {
                    if message.len() <= MESSAGE_CHARACTER_LIMIT {
                        Ok(Action::Create(message.to_string()))
                    } else {
                        Err(PromptParseError::MessageInvalid(
                            MESSAGE_INVALID_REASON_MESSAGE.to_string(),
                        ))
                    }
                }
                None => Err(PromptParseError::MessageInvalid(
                    MESSAGE_INVALID_REASON_MESSAGE.to_string(),
                )),
            },
            code => {
                let regex = Regex::new(r"^[a-z]+$").unwrap();
                if regex.is_match(code) {
                    Ok(Action::Read(code.to_owned()))
                } else {
                    Err(PromptParseError::MalformedAction)
                }
            }
        },
        None => Err(PromptParseError::MalformedAction),
    }
}
