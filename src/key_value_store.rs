use async_trait::async_trait;
use wasmbus_rpc::actor::prelude::RpcResult;

pub mod actor;
pub use actor::Actor;

pub mod in_memory;
pub use in_memory::InMemory;

#[async_trait]
pub trait KeyValueStore {
    async fn get(&mut self, key: &str) -> RpcResult<Option<String>>;
    async fn set(
        &mut self,
        key: &str,
        value: &str,
        expires: u32,
    ) -> RpcResult<()>;
    async fn incr_by(&mut self, key: &str, value: i32) -> RpcResult<i32>;
}
