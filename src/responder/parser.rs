use regex::Regex;
use std::result;

pub(crate) enum Action {
    Create(String),
    Read(String),
}

pub(crate) enum PromptParseError {
    MalformedAction,
    MessageMissing,
}

type PromptParseResult = result::Result<Action, PromptParseError>;
const PROMPT_REGEX: &str = r"\A([a-zA-Z]+)(\s+.*)?\z";
const CREATE_PROMPT_WORD: &str = "verbalcode";

pub(crate) fn parse(prompt: String) -> PromptParseResult {
    let exp = Regex::new(PROMPT_REGEX).unwrap();
    let mut matches = exp.captures_iter(prompt.trim());

    match matches.next() {
        Some(captures) => {
            let code = captures[1].to_lowercase();

            match code.as_str() {
                CREATE_PROMPT_WORD => {
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
