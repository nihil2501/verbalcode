use crate::key_value_store::KeyValueStore;
use std::result;
use wasmbus_rpc::actor::prelude::*;

// Codes last for a day.
const CODE_EXPIRY: u32 = 86_400;

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

// `CODE_LIST` shouldn't change over time in order for `CODE_LIST_INDEX_KEY` to
// remain coherent with respect to it.
const CODE_LIST: &[&str] = &["hello", "goodbye"];
const CODE_LIST_INDEX_KEY: &str = "verbalcode:code_list_index";

async fn generate_code<T: KeyValueStore>(store: &mut T) -> GenerateCodeResult {
    // Depends on atomic increment. Our relaxed strategy (one we still need
    // to prove is suitable enough) is as follows:
    //
    // 1. Increment an index into codeword list.
    let index = store.incr_by(CODE_LIST_INDEX_KEY, 1).await?;
    let index = (index as usize) % CODE_LIST.len();

    // 2. From index, fetch candidate codeword and check if it exists.
    // Expiry handled by kv-store.
    let code = CODE_LIST[index];
    let response = store.get(code).await?;
    match response {
        // If not, use it.
        None => Ok(code.to_string()),
        // Otherwise, error out with a `OverCapacity` and also decrement
        // back.
        Some(_) => {
            store.incr_by(CODE_LIST_INDEX_KEY, -1).await?;
            Err(GenerateCodeError::OverCapacity)
        }
    }
}

pub type CreateResult = result::Result<String, CreateError>;
pub enum CreateError {
    OverCapacity,
    Unknown(RpcError),
}

pub type FindResult = result::Result<String, FindError>;
pub enum FindError {
    NotFound,
    Unknown(RpcError),
}
pub type GenerateCodeResult = result::Result<String, GenerateCodeError>;
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
