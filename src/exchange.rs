use std::result;

// Codes last for a day.
const CODE_EXPIRY: u32 = 86_400;

pub async fn create<T: KeyValueStore>(
    message: String,
    store: T,
) -> CreateResult {
    let code = generate_code(&store).await?;
    store.set(&code, &message, CODE_EXPIRY).await?;
    Ok(code)
}

pub async fn find<T: KeyValueStore>(code: String, store: T) -> FindResult {
    let response = store.get(&code).await?;
    match response {
        Some(message) => Ok(message),
        None => Err(FindError::NotFound),
    }
}

const CODE_LIST: &[&str] = &["hello", "goodbye"];
const CODE_LIST_INDEX_KEY: &str = "code_list_index";

async fn generate_code<T: KeyValueStore>(store: &T) -> GenerateCodeResult {
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

#[async_trait]
pub trait KeyValueStore {
    async fn get(&self, key: &str) -> RpcResult<Option<String>>;
    async fn set(&self, key: &str, value: &str, expires: u32) -> RpcResult<()>;
    async fn incr_by(&self, key: &str, value: i32) -> RpcResult<i32>;
}

use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_keyvalue::{
    IncrementRequest, KeyValue, KeyValueSender, SetRequest,
};

pub struct KeyValueStoreActor<'a> {
    ctx: &'a Context,
}

impl KeyValueStoreActor<'_> {
    pub fn new(ctx: &Context) -> KeyValueStoreActor {
        KeyValueStoreActor { ctx }
    }
}

#[cfg(not(test))]
#[async_trait]
impl KeyValueStore for KeyValueStoreActor<'_> {
    // Interpretation of `Option` value might only be accurate for redis.
    async fn get(&self, code: &str) -> RpcResult<Option<String>> {
        let response = KeyValueSender::new().get(self.ctx, code).await?;
        if response.exists {
            Ok(Some(response.value))
        } else {
            Ok(None)
        }
    }

    async fn set(&self, key: &str, value: &str, expires: u32) -> RpcResult<()> {
        KeyValueSender::new()
            .set(
                self.ctx,
                &SetRequest {
                    key: key.to_owned(),
                    value: value.to_owned(),
                    expires,
                },
            )
            .await
    }

    async fn incr_by(&self, key: &str, value: i32) -> RpcResult<i32> {
        KeyValueSender::new()
            .increment(
                self.ctx,
                &IncrementRequest {
                    key: key.to_string(),
                    value,
                },
            )
            .await
    }
}

#[cfg(test)]
pub mod test {
    use crate::exchange::KeyValueStore;
    use async_trait::async_trait;
    use wasmbus_rpc::actor::prelude::*;

    pub struct MockKeyValueStore;

    #[async_trait]
    impl KeyValueStore for MockKeyValueStore {
        async fn get(&self, key: &str) -> RpcResult<Option<String>> {
            todo!()
        }

        async fn set(
            &self,
            key: &str,
            value: &str,
            expires: u32,
        ) -> RpcResult<()> {
            todo!()
        }

        async fn incr_by(&self, key: &str, value: i32) -> RpcResult<i32> {
            todo!()
        }
    }
}
