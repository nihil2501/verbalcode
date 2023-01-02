use crate::key_value_store::KeyValueStore;
use std::result;
use tokio::time::Duration;
use wasmbus_rpc::actor::prelude::*;

// Codes last for a day.
const CODE_EXPIRY: Duration = Duration::from_secs(86_400);

pub async fn create<T: KeyValueStore>(
    message: String,
    store: &mut T,
) -> CreateResult {
    let code = generate_code(store).await?;
    store.set(&code, &message, CODE_EXPIRY).await?;
    Ok(code)
}

pub async fn find<T: KeyValueStore>(code: String, store: &mut T) -> FindResult {
    let response = store.get(&code).await?;
    match response {
        Some(message) => Ok(message),
        None => Err(FindError::NotFound),
    }
}

mod codes;
// `CODES` shouldn't change over time in order for `partyskunk:codes_index` to
// remain coherent with respect to it.
use codes::CODES;

const CODES_INDEX_KEY: &str = "partyskunk:codes_index";

async fn generate_code<T: KeyValueStore>(store: &mut T) -> GenerateCodeResult {
    // Depends on atomic increment. Our relaxed strategy (one we still need
    // to prove is suitable enough) is as follows:
    //
    // 1. Increment an index into codeword list.
    let index = store.incr_by(CODES_INDEX_KEY, 1).await?;
    let index = (index as usize) % CODES.len();

    // 2. From index, fetch candidate codeword and check if it exists.
    // Expiry handled by kv-store.
    let code = CODES[index];
    let response = store.get(code).await?;
    match response {
        // If not, use it.
        None => Ok(code.to_string()),
        // Otherwise, error out with a `OverCapacity` and also decrement
        // back.
        Some(_) => {
            store.incr_by(CODES_INDEX_KEY, -1).await?;
            Err(GenerateCodeError::OverCapacity)
        }
    }
}

pub type FindResult = result::Result<String, FindError>;
pub type GenerateCodeResult = result::Result<String, GenerateCodeError>;
pub type CreateResult = result::Result<String, CreateError>;

#[derive(Debug)]
pub enum CreateError {
    OverCapacity,
    Unknown(RpcError),
}

#[derive(Debug)]
pub enum FindError {
    NotFound,
    Unknown(RpcError),
}

#[derive(Debug)]
pub enum GenerateCodeError {
    OverCapacity,
    Unknown(RpcError),
}

impl From<GenerateCodeError> for CreateError {
    fn from(error: GenerateCodeError) -> Self {
        match error {
            GenerateCodeError::OverCapacity => CreateError::OverCapacity,
            GenerateCodeError::Unknown(error) => CreateError::Unknown(error),
        }
    }
}

impl From<RpcError> for CreateError {
    fn from(error: RpcError) -> Self {
        CreateError::Unknown(error)
    }
}

impl From<RpcError> for FindError {
    fn from(error: RpcError) -> Self {
        FindError::Unknown(error)
    }
}

impl From<RpcError> for GenerateCodeError {
    fn from(error: RpcError) -> Self {
        GenerateCodeError::Unknown(error)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::key_value_store;

    #[tokio::test]
    async fn it_exchanges_limited_code_words_with_expiry() {
        tokio::time::pause();
        let mut store = key_value_store::InMemory::new();

        let result = find("hello".to_string(), &mut store).await;
        assert!(matches!(result, Err(FindError::NotFound)));

        let result = create("message 1".to_string(), &mut store).await;
        assert_eq!(result.unwrap(), "hello");

        let result = find("hello".to_string(), &mut store).await;
        assert_eq!(result.unwrap(), "message 1".to_string());

        let result = create("message 2".to_string(), &mut store).await;
        assert_eq!(result.unwrap(), "goodbye");

        let result = create("message 3".to_string(), &mut store).await;
        assert!(matches!(result, Err(CreateError::OverCapacity)));

        tokio::time::advance(CODE_EXPIRY).await;

        let result = create("message 3".to_string(), &mut store).await;
        assert_eq!(result.unwrap(), "hello");

        let result = find("goodbye".to_string(), &mut store).await;
        assert!(matches!(result, Err(FindError::NotFound)));
    }
}
