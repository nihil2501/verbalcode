use async_trait::async_trait;
use std::result;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_keyvalue::{
    IncrementRequest, KeyValue, KeyValueSender, SetRequest,
};

#[async_trait]
pub trait CodeExchange {
    async fn create(self, message: String) -> CreateResult;
    async fn find(self, code: String) -> FindResult;
}

pub type CreateResult = result::Result<String, CreateError>;

pub enum CreateError {
    Invalid(String),
    DatabaseFull,
    Other(RpcError),
}

impl From<RpcError> for CreateError {
    fn from(error: RpcError) -> Self {
        CreateError::Other(error)
    }
}

impl From<GenerateCodeError> for CreateError {
    fn from(error: GenerateCodeError) -> Self {
        CreateError::DatabaseFull
    }
}

pub type FindResult = result::Result<String, FindError>;

pub enum FindError {
    NotFound,
    Other(RpcError),
}

impl From<RpcError> for FindError {
    fn from(error: RpcError) -> Self {
        FindError::Other(error)
    }
}

pub struct ActorCodeExchange<'a> {
    ctx: &'a Context,
}

// codes last for a day
const CODE_EXPIRY: u32 = 86_400;

#[async_trait]
impl CodeExchange for ActorCodeExchange<'_> {
    async fn create(self, message: String) -> CreateResult {
        let key = self.generate_code().await?;
        KeyValueSender::new()
            .set(
                self.ctx,
                &SetRequest {
                    key,
                    value: message,
                    expires: CODE_EXPIRY,
                },
            )
            .await?;

        Ok(key)
    }

    async fn find(self, code: String) -> FindResult {
        let response = KeyValueSender::new().get(self.ctx, &code).await?;
        if response.exists {
            Ok(response.value)
        } else {
            Err(FindError::NotFound)
        }
    }
}

type GenerateCodeResult = result::Result<String, GenerateCodeError>;

struct GenerateCodeError;

impl From<RpcError> for GenerateCodeError {
    fn from(error: RpcError) -> Self {
        GenerateCodeError
    }
}

const CODE_LIST_INDEX_KEY: &str = "code_list_index";
const CODE_LIST: &[&str] = &["hello", "goodbye"];

impl ActorCodeExchange<'_> {
    pub fn new(ctx: &Context) -> ActorCodeExchange {
        ActorCodeExchange { ctx }
    }

    async fn generate_code(self) -> GenerateCodeResult {
        // depending on atomic increment. algo is to:
        // 1. increment a key indexing into codeword list
        let index = KeyValueSender::new()
            .increment(
                self.ctx,
                &IncrementRequest {
                    key: CODE_LIST_INDEX_KEY.to_string(),
                    value: 1,
                },
            )
            .await?;

        // 2. from index, fetch candidate codeword
        let index = (index as usize) % CODE_LIST.len();
        let code = CODE_LIST[index];

        // 3. check if codeword is free (expiry handled by kv-store). the combo
        // of that fact with this approach is our relaxed strategy.
        let taken = KeyValueSender::new().contains(self.ctx, &code).await?;

        // 4. if yes, use it, otherwise error out with a `DatabaseFull`
        if taken {
            Ok(code.to_string())
        } else {
            Err(GenerateCodeError)
        }
    }
}
