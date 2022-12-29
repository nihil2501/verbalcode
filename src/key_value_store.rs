use async_trait::async_trait;
use tokio::time::Duration;
use wasmbus_rpc::actor::prelude::RpcResult;

mod actor;
#[cfg(target_arch = "wasm32")]
pub use actor::Actor;

mod in_memory;
pub use in_memory::InMemory;

#[async_trait]
pub trait KeyValueStore {
    async fn get(&mut self, key: &str) -> RpcResult<Option<String>>;
    async fn set(
        &mut self,
        key: &str,
        value: &str,
        expires: Duration,
    ) -> RpcResult<()>;
    async fn incr_by(&mut self, key: &str, value: i32) -> RpcResult<i32>;
}
