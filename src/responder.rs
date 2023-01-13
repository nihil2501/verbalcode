use crate::key_value_store::KeyValueStore;

use crate::logger;
mod exchange;
mod messages;
mod parser;

pub async fn handle<T: KeyValueStore>(
    prompt: String,
    prompter: String,
    store: &mut T,
) -> String {
    logger::log(format!("prompter: {}, prompt: {}", prompter, prompt)).await;

    // Prompt can either parse successfully or not.
    match parser::parse(prompt) {
        // When prompt does parse correctly it is for one of a distinct set of
        // actions.
        Ok(action) => match action {
            // Prompt indicates that a code should be created for some message.
            parser::Action::Create(message) => {
                let result = create(message, store).await;
                match result {
                    // Create is valid, yielding back a code corresponding to
                    // the message.
                    Ok(code) => messages::create_success(code),

                    Err(error) => match error {
                        // All code words are used up.
                        exchange::CreateError::OverCapacity => {
                            messages::create_over_capacity_error()
                        }
                        // Unknown error.
                        exchange::CreateError::Unknown(_) => {
                            messages::create_unknown_error()
                        }
                    },
                }
            }

            // Prompt indicates that a code should be read.
            parser::Action::Read(code) => {
                let result = find(code, store).await;
                match result {
                    // Code exists in the exchange, yielding back the
                    // corresponding message.
                    Ok(message) => messages::find_success(message),

                    Err(error) => match error {
                        // Code doesn't exist in the exchange.
                        exchange::FindError::NotFound => {
                            messages::find_not_found_error()
                        }
                        // Unknown error.
                        exchange::FindError::Unknown(_) => {
                            messages::find_unknown_error()
                        }
                    },
                }
            }
        },

        // When prompt doesn't parse correctly it does so in one of these ways.
        Err(error) => match error {
            // Prompt is so malformed it fails to indicate any action.
            parser::PromptParseError::MalformedAction => {
                messages::prompt_malformed_error()
            }

            // Prompt indicates a create but message is too long or short.
            parser::PromptParseError::MessageInvalid(reason) => {
                messages::prompt_create_message_invalid_error(reason)
            }
        },
    }
}

#[cfg(not(test))]
async fn create<T: KeyValueStore>(
    message: String,
    store: &mut T,
) -> Result<String, exchange::CreateError> {
    exchange::create(message, store).await
}

#[cfg(not(test))]
async fn find<T: KeyValueStore>(
    code: String,
    store: &mut T,
) -> Result<String, exchange::FindError> {
    exchange::find(code, store).await
}

#[cfg(test)]
use indoc::indoc;
#[cfg(test)]
use wasmbus_rpc::actor::prelude::*;

#[cfg(test)]
async fn create<T: KeyValueStore>(
    message: String,
    _store: &mut T,
) -> Result<String, exchange::CreateError> {
    match message.as_str() {
        "valid message" => Ok("validcode".to_string()),
        indoc! {"
            valid message
            spanning lines"} => Ok("validcode".to_string()),
        "over capacity" => Err(exchange::CreateError::OverCapacity),
        "unknown error" => Err(exchange::CreateError::Unknown(
            RpcError::Other("unknown".to_string()),
        )),
        _ => panic!(),
    }
}

#[cfg(test)]
async fn find<T: KeyValueStore>(
    code: String,
    _store: &mut T,
) -> Result<String, exchange::FindError> {
    match code.as_str() {
        "foundcode" => Ok("found message".to_string()),
        "notfoundcode" => Err(exchange::FindError::NotFound),
        "unknownerror" => Err(exchange::FindError::Unknown(RpcError::Other(
            "unknown".to_string(),
        ))),
        _ => panic!(),
    }
}

#[cfg(test)]
pub mod test {
    use crate::{key_value_store, responder::*};

    #[tokio::test]
    async fn create_success() {
        let response = handle(
            "partyskunk valid message".to_string(),
            "prompter".to_string(),
            &mut mock_key_value_store(),
        )
        .await;

        assert_eq!(response, messages::create_success("validcode".to_string()))
    }

    #[tokio::test]
    async fn create_success_multiline() {
        let response = handle(
            indoc! {"
                partyskunk valid message
                spanning lines
            "}
            .to_string(),
            "prompter".to_string(),
            &mut mock_key_value_store(),
        )
        .await;

        assert_eq!(response, messages::create_success("validcode".to_string()))
    }

    #[tokio::test]
    async fn create_over_capacity_error() {
        let response = handle(
            "partyskunk over capacity".to_string(),
            "prompter".to_string(),
            &mut mock_key_value_store(),
        )
        .await;

        assert_eq!(response, messages::create_over_capacity_error())
    }

    #[tokio::test]
    async fn create_unknown_error() {
        let response = handle(
            "partyskunk unknown error".to_string(),
            "prompter".to_string(),
            &mut mock_key_value_store(),
        )
        .await;

        assert_eq!(response, messages::create_unknown_error())
    }

    #[tokio::test]
    async fn find_success() {
        let response = handle(
            "foundcode".to_string(),
            "prompter".to_string(),
            &mut mock_key_value_store(),
        )
        .await;

        assert_eq!(
            response,
            messages::find_success("found message".to_string())
        )
    }

    #[tokio::test]
    async fn find_not_found_error() {
        let response = handle(
            "notfoundcode".to_string(),
            "prompter".to_string(),
            &mut mock_key_value_store(),
        )
        .await;

        assert_eq!(response, messages::find_not_found_error())
    }

    #[tokio::test]
    async fn find_unknown_error() {
        let response = handle(
            "unknownerror".to_string(),
            "prompter".to_string(),
            &mut mock_key_value_store(),
        )
        .await;

        assert_eq!(response, messages::find_unknown_error())
    }

    #[tokio::test]
    async fn prompt_malformed() {
        let response = handle(
            "partyskunk!".to_string(),
            "prompter".to_string(),
            &mut mock_key_value_store(),
        )
        .await;

        assert_eq!(response, messages::prompt_malformed_error());
    }

    #[tokio::test]
    async fn prompt_create_message_invalid_error() {
        let response = handle(
            "partyskunk".to_string(),
            "prompter".to_string(),
            &mut mock_key_value_store(),
        )
        .await;

        assert_eq!(
            response,
            messages::prompt_create_message_invalid_error(
                "some invalid reason".to_string()
            )
        )
    }

    fn mock_key_value_store() -> key_value_store::InMemory {
        key_value_store::InMemory::new()
    }
}
