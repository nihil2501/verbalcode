mod code_exchange;
mod parser;
mod response;

pub fn handle(prompt: String, prompter: String) -> String {
    // For now we just log prompter (but not in wasmcloud because of async
    // restriction to their logging crate).
    println!("prompt: {}, prompter: {}", prompt, prompter);

    // Prompt can either parse successfully or not.
    match parser::parse(prompt) {
        // When prompt does parse correctly it is for one of a distinct set of
        // actions.
        Ok(action) => match action {
            // Prompt indicates that a code should be created for some message.
            parser::Action::Create(message) => match create(message) {
                // Create is valid, yielding back a code corresponding to the
                // message.
                Ok(code) => response::create_valid(code),

                Err(error) => match error {
                    // Message is invalid for some reason.
                    code_exchange::CreateError::Invalid(reason) => {
                        response::create_invalid(reason)
                    }
                },
            },

            // Prompt indicates that a code should be read.
            parser::Action::Read(code) => match find(code) {
                // Code exists in the exchange, yielding back the corresponding
                // message.
                Ok(message) => response::find_found(message),

                Err(error) => match error {
                    // Code doesn't exist in the exchange.
                    code_exchange::FindError::NotFound => {
                        response::find_not_found()
                    }
                },
            },
        },

        // When prompt doesn't parse correctly it does so in one of these ways.
        Err(error) => match error {
            // Prompt is so malformed it fails to indicate any action.
            parser::PromptParseError::MalformedAction => {
                response::prompt_malformed()
            }

            // Prompt indicates a create but any kind of message argument is
            // absent.
            parser::PromptParseError::MessageMissing => {
                response::prompt_create_message_missing()
            }
        },
    }
}

#[cfg(not(test))]
fn find(code: String) -> code_exchange::FindResult {
    code_exchange::find(code)
}

#[cfg(not(test))]
fn create(message: String) -> code_exchange::CreateResult {
    code_exchange::create(message)
}

#[cfg(test)]
fn find(code: String) -> code_exchange::FindResult {
    match code.as_str() {
        "foundcode" => Ok("found message".to_string()),
        "notfoundcode" => Err(code_exchange::FindError::NotFound),
        _ => panic!(),
    }
}

#[cfg(test)]
fn create(message: String) -> code_exchange::CreateResult {
    match message.as_str() {
        "valid message" => Ok("validcode".to_string()),
        "invalid message" => Err(code_exchange::CreateError::Invalid(
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

        assert_eq!(response, response::create_valid("validcode".to_string()))
    }

    #[test]
    fn create_invalid() {
        let response = handle(
            "verbalcode invalid message".to_string(),
            "prompter".to_string(),
        );

        assert_eq!(
            response,
            response::create_invalid("invalid reason".to_string())
        )
    }

    #[test]
    fn find_found() {
        let response = handle("foundcode".to_string(), "prompter".to_string());

        assert_eq!(response, response::find_found("found message".to_string()))
    }

    #[test]
    fn find_not_found() {
        let response =
            handle("notfoundcode".to_string(), "prompter".to_string());

        assert_eq!(response, response::find_not_found())
    }

    #[test]
    fn prompt_malformed() {
        let response =
            handle("verbalcode!".to_string(), "prompter".to_string());

        assert_eq!(response, response::prompt_malformed());
    }

    #[test]
    fn prompt_create_message_missing() {
        let response =
            handle("verbalcode ".to_string(), "prompter".to_string());

        assert_eq!(response, response::prompt_create_message_missing())
    }
}
