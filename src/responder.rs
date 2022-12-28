use crate::exchange;
mod parser;
mod response;

pub async fn handle<T: exchange::KeyValueStore>(
    prompt: String,
    _prompter: String,
    store: &mut T,
) -> String {
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
                    Ok(code) => response::create_valid(code),

                    Err(error) => match error {
                        // All code words are used up.
                        exchange::CreateError::OverCapacity => {
                            response::create_over_capacity()
                        }
                        // Unknown error.
                        exchange::CreateError::Unknown(_) => {
                            response::create_unknown_error()
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
                    Ok(message) => response::find_found(message),

                    Err(error) => match error {
                        // Code doesn't exist in the exchange.
                        exchange::FindError::NotFound => {
                            response::find_not_found()
                        }
                        // Unknown error.
                        exchange::FindError::Unknown(_) => {
                            response::find_unknown_error()
                        }
                    },
                }
            }
        },

        // When prompt doesn't parse correctly it does so in one of these ways.
        Err(error) => match error {
            // Prompt is so malformed it fails to indicate any action.
            parser::PromptParseError::MalformedAction => {
                response::prompt_malformed()
            }

            // Prompt indicates a create but message is too long or short.
            parser::PromptParseError::MessageInvalid(reason) => {
                response::prompt_create_message_invalid(reason)
            }
        },
    }
}

#[cfg(not(test))]
async fn create<T: exchange::KeyValueStore>(
    message: String,
    store: &mut T,
) -> Result<String, exchange::CreateError> {
    exchange::create(message, store).await
}

#[cfg(not(test))]
async fn find<T: exchange::KeyValueStore>(
    code: String,
    store: &mut T,
) -> Result<String, exchange::FindError> {
    exchange::find(code, store).await
}

#[cfg(test)]
use wasmbus_rpc::actor::prelude::*;

#[cfg(test)]
async fn create<T: exchange::KeyValueStore>(
    message: String,
    _store: &mut T,
) -> Result<String, exchange::CreateError> {
    match message.as_str() {
        "valid message" => Ok("validcode".to_string()),
        "over capacity" => Err(exchange::CreateError::OverCapacity),
        "unknown error" => Err(exchange::CreateError::Unknown(
            RpcError::Other("unknown".to_string()),
        )),
        _ => panic!(),
    }
}

#[cfg(test)]
async fn find<T: exchange::KeyValueStore>(
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
    use crate::responder::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn create_valid() {
        let response = handle(
            "verbalcode valid message".to_string(),
            "prompter".to_string(),
            &mut mock_key_value_store(),
        )
        .await;

        assert_eq!(response, response::create_valid("validcode".to_string()))
    }

    fn mock_key_value_store() -> HashMap<String, String> {
        HashMap::new()
    }

    #[tokio::test]
    async fn create_over_capacity() {
        let response = handle(
            "verbalcode over capacity".to_string(),
            "prompter".to_string(),
            &mut mock_key_value_store(),
        )
        .await;

        assert_eq!(response, response::create_over_capacity())
    }

    #[tokio::test]
    async fn create_unknown_error() {
        let response = handle(
            "verbalcode unknown error".to_string(),
            "prompter".to_string(),
            &mut mock_key_value_store(),
        )
        .await;

        assert_eq!(response, response::create_unknown_error())
    }

    #[tokio::test]
    async fn find_found() {
        let response = handle(
            "foundcode".to_string(),
            "prompter".to_string(),
            &mut mock_key_value_store(),
        )
        .await;

        assert_eq!(response, response::find_found("found message".to_string()))
    }

    #[tokio::test]
    async fn find_not_found() {
        let response = handle(
            "notfoundcode".to_string(),
            "prompter".to_string(),
            &mut mock_key_value_store(),
        )
        .await;

        assert_eq!(response, response::find_not_found())
    }

    #[tokio::test]
    async fn find_unknown_error() {
        let response = handle(
            "unknownerror".to_string(),
            "prompter".to_string(),
            &mut mock_key_value_store(),
        )
        .await;

        assert_eq!(response, response::find_unknown_error())
    }

    #[tokio::test]
    async fn prompt_malformed() {
        let response = handle(
            "verbalcode!".to_string(),
            "prompter".to_string(),
            &mut mock_key_value_store(),
        )
        .await;

        assert_eq!(response, response::prompt_malformed());
    }

    #[tokio::test]
    async fn prompt_create_message_invalid() {
        let response = handle(
            "verbalcode".to_string(),
            "prompter".to_string(),
            &mut mock_key_value_store(),
        )
        .await;

        assert_eq!(
            response,
            response::prompt_create_message_invalid(
                "some invalid reason".to_string()
            )
        )
    }
}
