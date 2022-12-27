use std::result;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_keyvalue::{
    GetResponse, IncrementRequest, KeyValue, KeyValueSender, SetRequest,
};

#[async_trait]
pub trait CodeExchange {
    async fn create(&self, message: String) -> CreateResult;
    async fn find(&self, code: String) -> FindResult;
}

// Codes last for a day.
const CODE_EXPIRY: u32 = 86_400;

#[async_trait]
impl CodeExchange for ActorCodeExchange<'_> {
    async fn create(&self, message: String) -> CreateResult {
        let code = self.generate_code().await?;
        self.set(&code, &message, CODE_EXPIRY).await?;
        Ok(code)
    }

    async fn find(&self, code: String) -> FindResult {
        let response = self.get(&code).await?;
        if response.exists {
            Ok(response.value)
        } else {
            Err(FindError::NotFound)
        }
    }
}

const CODE_LIST: &[&str] = &["hello", "goodbye"];
const CODE_LIST_INDEX_KEY: &str = "code_list_index";

impl ActorCodeExchange<'_> {
    async fn generate_code(&self) -> GenerateCodeResult {
        // Depends on atomic increment. Our relaxed strategy (one we still need
        // to prove out that it is suitable enough) is as follows:
        //
        // 1. Increment an index into codeword list.
        let index = self.incr_by(CODE_LIST_INDEX_KEY, 1).await?;
        let index = (index as usize) % CODE_LIST.len();

        // 2. From index, fetch candidate codeword and check if it exists.
        // Expiry handled by kv-store
        let code = CODE_LIST[index];
        let response = self.get(code).await?;
        if response.exists {
            // If yes, error out with a `OverCapacity` and also decrement back.
            self.incr_by(CODE_LIST_INDEX_KEY, -1).await?;
            Err(GenerateCodeError::OverCapacity)
        } else {
            // Otherwise use it.
            Ok(code.to_string())
        }
    }
}

// Just some helpers so things are easier to look at.
impl ActorCodeExchange<'_> {
    pub fn new(ctx: &Context) -> ActorCodeExchange {
        ActorCodeExchange { ctx }
    }

    async fn get(&self, code: &str) -> RpcResult<GetResponse> {
        KeyValueSender::new().get(self.ctx, code).await
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

pub type CreateResult = result::Result<String, CreateError>;

pub enum CreateError {
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

pub type FindResult = result::Result<String, FindError>;

pub enum FindError {
    NotFound,
    Unknown(RpcError),
}

impl From<RpcError> for FindError {
    fn from(error: RpcError) -> Self {
        FindError::Unknown(error)
    }
}

pub struct ActorCodeExchange<'a> {
    ctx: &'a Context,
}

type GenerateCodeResult = result::Result<String, GenerateCodeError>;

enum GenerateCodeError {
    OverCapacity,
    Unknown(RpcError),
}

impl From<RpcError> for GenerateCodeError {
    fn from(error: RpcError) -> Self {
        GenerateCodeError::Unknown(error)
    }
}
