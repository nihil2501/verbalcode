#![cfg(target_arch = "wasm32")]

use tokio::time::Duration;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_keyvalue::{
    IncrementRequest, KeyValue, KeyValueSender, SetRequest,
};

use super::KeyValueStore;

pub struct Actor<'a> {
    ctx: &'a Context,
}

impl Actor<'_> {
    pub fn new(ctx: &Context) -> Actor {
        Actor { ctx }
    }
}

#[async_trait]
impl KeyValueStore for Actor<'_> {
    // Interpretation of `Option` value might only be accurate for redis.
    async fn get(&mut self, code: &str) -> RpcResult<Option<String>> {
        let response = KeyValueSender::new().get(self.ctx, code).await?;
        if response.exists {
            Ok(Some(response.value))
        } else {
            Ok(None)
        }
    }

    async fn set(
        &mut self,
        key: &str,
        value: &str,
        expires: Duration,
    ) -> RpcResult<()> {
        KeyValueSender::new()
            .set(
                self.ctx,
                &SetRequest {
                    key: key.to_owned(),
                    value: value.to_owned(),
                    expires: expires.as_secs().try_into().unwrap(),
                },
            )
            .await
    }

    async fn incr_by(&mut self, key: &str, value: i32) -> RpcResult<i32> {
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
