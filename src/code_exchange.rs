use std::result;

#[async_trait]
trait CodeExchange {
    async fn create(&self, message: String) -> CreateResult;
    async fn find(&self, code: String) -> FindResult;
    // TODO: reconsider
    async fn generate_code(&self) -> GenerateCodeResult;
}

type CreateResult = result::Result<String, CreateError>;
enum CreateError {
    OverCapacity,
    Unknown(RpcError),
}

type FindResult = result::Result<String, FindError>;
enum FindError {
    NotFound,
    Unknown(RpcError),
}

// Codes last for a day.
const CODE_EXPIRY: u32 = 86_400;
const CODE_LIST: &[&str] = &["hello", "goodbye"];
const CODE_LIST_INDEX_KEY: &str = "code_list_index";

#[async_trait]
impl<T: KeyValueStore> CodeExchange for T {
    async fn create(&self, message: String) -> CreateResult {
        let code = self.generate_code().await?;
        self.set(&code, &message, CODE_EXPIRY).await?;
        Ok(code)
    }

    async fn find(&self, code: String) -> FindResult {
        let response = self.get(&code).await?;
        match response {
            Some(message) => Ok(message),
            None => Err(FindError::NotFound),
        }
    }

    async fn generate_code(&self) -> GenerateCodeResult {
        // Depends on atomic increment. Our relaxed strategy (one we still need
        // to prove out that it is suitable enough) is as follows:
        //
        // 1. Increment an index into codeword list.
        let index = self.incr_by(CODE_LIST_INDEX_KEY, 1).await?;
        let index = (index as usize) % CODE_LIST.len();

        // 2. From index, fetch candidate codeword and check if it exists.
        // Expiry handled by kv-store.
        let code = CODE_LIST[index];
        let response = self.get(code).await?;
        match response {
            // If not, use it.
            None => Ok(code.to_string()),
            // Otherwise, error out with a `OverCapacity` and also decrement
            // back.
            Some(_) => {
                self.incr_by(CODE_LIST_INDEX_KEY, -1).await?;
                Err(GenerateCodeError::OverCapacity)
            }
        }
    }
}

type GenerateCodeResult = result::Result<String, GenerateCodeError>;
enum GenerateCodeError {
    OverCapacity,
    Unknown(RpcError),
}

impl From<RpcError> for CreateError {
    fn from(error: RpcError) -> Self {
        CreateError::Unknown(error)
    }
}

impl From<GenerateCodeError> for CreateError {
    fn from(error: GenerateCodeError) -> Self {
        match error {
            GenerateCodeError::OverCapacity => CreateError::OverCapacity,
            GenerateCodeError::Unknown(error) => CreateError::Unknown(error),
        }
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
trait KeyValueStore {
    async fn get(&self, key: &str) -> RpcResult<Option<String>>;
    async fn set(&self, key: &str, value: &str, expires: u32) -> RpcResult<()>;
    async fn incr_by(&self, key: &str, value: i32) -> RpcResult<i32>;
}

use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_keyvalue::{
    IncrementRequest, KeyValue, KeyValueSender, SetRequest,
};

struct KeyValueStoreActor<'a> {
    ctx: &'a Context,
}

impl KeyValueStoreActor<'_> {
    pub fn new(ctx: &Context) -> KeyValueStoreActor {
        KeyValueStoreActor { ctx }
    }
}

#[async_trait]
impl KeyValueStore for KeyValueStoreActor<'_> {
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
