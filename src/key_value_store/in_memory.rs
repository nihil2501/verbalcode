use super::KeyValueStore;
use async_trait::async_trait;
use std::collections::HashMap;
use tokio::time::{Duration, Instant};
use wasmbus_rpc::actor::prelude::*;

#[derive(Debug)]
pub struct InMemory {
    map: HashMap<String, String>,
    expiry: HashMap<String, Instant>,
}

impl InMemory {
    #[allow(dead_code)] // Just to settle `cfg` confusion.
    pub fn new() -> InMemory {
        InMemory {
            map: HashMap::new(),
            expiry: HashMap::new(),
        }
    }
}

#[async_trait]
impl KeyValueStore for InMemory {
    async fn get(&mut self, key: &str) -> RpcResult<Option<String>> {
        let value = match self.expiry.get(key) {
            None => None,
            Some(time) => {
                if time > &Instant::now() {
                    self.map.get(key)
                } else {
                    self.expiry.remove(key);
                    self.map.remove(key);
                    None
                }
            }
        };

        Ok(value.cloned())
    }

    async fn set(
        &mut self,
        key: &str,
        value: &str,
        expires: Duration,
    ) -> RpcResult<()> {
        let expires = Instant::now() + expires;
        self.expiry.insert(key.to_string(), expires);
        self.map.insert(key.to_string(), value.to_string());

        Ok(())
    }

    async fn incr_by(&mut self, key: &str, value: i32) -> RpcResult<i32> {
        let value = match self.map.get(key) {
            Some(i) => i.parse::<i32>().unwrap() + value,
            None => 0,
        };

        self.map.insert(key.to_string(), value.to_string());

        Ok(value)
    }
}
