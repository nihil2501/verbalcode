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
const PROMPT_REGEX: &str = r"\A([a-zA-Z]+)(\s+(?s:.)*)?\z";
const CREATE_PROMPT_WORD: &str = "partyskunk";
const MESSAGE_CHARACTER_LIMIT: usize = 140;
const MESSAGE_INVALID_REASON_MESSAGE: &str =
    "Message must not be empty and must be less than 140 characters.";

pub fn parse(prompt: String) -> PromptParseResult {
    let exp = Regex::new(PROMPT_REGEX).unwrap();
    let mut matches = exp.captures_iter(prompt.trim());

    match matches.next() {
        Some(captures) => {
            let code = captures[1].to_lowercase();

            match code.as_str() {
                CREATE_PROMPT_WORD => {
                    let message =
                        captures.get(2).map_or("", |c| c.as_str().trim());

                    let len = message.len();
                    if len > 0 && len <= MESSAGE_CHARACTER_LIMIT {
                        Ok(Action::Create(message.to_string()))
                    } else {
                        Err(PromptParseError::MessageInvalid(
                            MESSAGE_INVALID_REASON_MESSAGE.to_string(),
                        ))
                    }
                }
                _ => Ok(Action::Read(code)),
            }
        }
        None => Err(PromptParseError::MalformedAction),
    }
}
