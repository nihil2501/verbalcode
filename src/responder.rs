use wasmcloud_interface_logging::log;

use crate::code_exchange;
mod parser;
mod response;

pub async fn handle<E: code_exchange::CodeExchange>(
    prompt: String,
    prompter: String,
    exchange: E,
) -> String {
    // For now we just log prompter.
    log(
        "debug",
        format!("prompt: {}, prompter: {}", prompt, prompter),
    )
    .await
    .iter()
    .next();

    // Prompt can either parse successfully or not.
    match parser::parse(prompt) {
        // When prompt does parse correctly it is for one of a distinct set of
        // actions.
        Ok(action) => match action {
            // Prompt indicates that a code should be created for some message.
            parser::Action::Create(message) => {
                let create_result = exchange.create(message).await;
                match create_result {
                    // Create is valid, yielding back a code corresponding to
                    // the message.
                    Ok(code) => response::create_valid(code),

                    Err(error) => match error {
                        // All code words are used up.
                        code_exchange::CreateError::OverCapacity => {
                            response::create_over_capacity()
                        }
                        // Unknown error.
                        code_exchange::CreateError::Unknown(_) => {
                            response::create_unknown_error()
                        }
                    },
                }
            }

            // Prompt indicates that a code should be read.
            parser::Action::Read(code) => {
                let find_result = exchange.find(code).await;
                match find_result {
                    // Code exists in the exchange, yielding back the
                    // corresponding message.
                    Ok(message) => response::find_found(message),

                    Err(error) => match error {
                        // Code doesn't exist in the exchange.
                        code_exchange::FindError::NotFound => {
                            response::find_not_found()
                        }
                        // Unknown error.
                        code_exchange::FindError::Unknown(_) => {
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

#[cfg(test)]
mod test {
    use crate::responder::*;
    use async_trait::async_trait;
    use wasmbus_rpc::error;

    struct MockCodeExchange;

    #[async_trait]
    impl code_exchange::CodeExchange for MockCodeExchange {
        async fn create(&self, message: String) -> code_exchange::CreateResult {
            match message.as_str() {
                "valid message" => Ok("validcode".to_string()),
                "over capacity" => {
                    Err(code_exchange::CreateError::OverCapacity)
                }
                "unknown error" => Err(code_exchange::CreateError::Unknown(
                    error::RpcError::Other("unknown".to_string()),
                )),
                _ => panic!(),
            }
        }

        async fn find(&self, code: String) -> code_exchange::FindResult {
            match code.as_str() {
                "foundcode" => Ok("found message".to_string()),
                "notfoundcode" => Err(code_exchange::FindError::NotFound),
                "unknownerror" => Err(code_exchange::FindError::Unknown(
                    error::RpcError::Other("unknown".to_string()),
                )),
                _ => panic!(),
            }
        }
    }

    #[tokio::test]
    async fn create_valid() {
        let response = handle(
            "verbalcode valid message".to_string(),
            "prompter".to_string(),
            MockCodeExchange,
        )
        .await;

        assert_eq!(response, response::create_valid("validcode".to_string()))
    }

    #[tokio::test]
    async fn create_over_capacity() {
        let response = handle(
            "verbalcode over capacity".to_string(),
            "prompter".to_string(),
            MockCodeExchange,
        )
        .await;

        assert_eq!(response, response::create_over_capacity())
    }

    #[tokio::test]
    async fn create_unknown_error() {
        let response = handle(
            "verbalcode unknown error".to_string(),
            "prompter".to_string(),
            MockCodeExchange,
        )
        .await;

        assert_eq!(response, response::create_unknown_error())
    }

    #[tokio::test]
    async fn find_found() {
        let response = handle(
            "foundcode".to_string(),
            "prompter".to_string(),
            MockCodeExchange,
        )
        .await;

        assert_eq!(response, response::find_found("found message".to_string()))
    }

    #[tokio::test]
    async fn find_not_found() {
        let response = handle(
            "notfoundcode".to_string(),
            "prompter".to_string(),
            MockCodeExchange,
        )
        .await;

        assert_eq!(response, response::find_not_found())
    }

    #[tokio::test]
    async fn find_unknown_error() {
        let response = handle(
            "unknownerror".to_string(),
            "prompter".to_string(),
            MockCodeExchange,
        )
        .await;

        assert_eq!(response, response::find_unknown_error())
    }

    #[tokio::test]
    async fn prompt_malformed() {
        let response = handle(
            "verbalcode!".to_string(),
            "prompter".to_string(),
            MockCodeExchange,
        )
        .await;

        assert_eq!(response, response::prompt_malformed());
    }

    #[tokio::test]
    async fn prompt_create_message_invalid() {
        let response = handle(
            "verbalcode".to_string(),
            "prompter".to_string(),
            MockCodeExchange,
        )
        .await;

        assert_eq!(
            response,
            response::prompt_create_message_invalid(
                "some invalid reason".to_string()
            )
        );
    }
}
