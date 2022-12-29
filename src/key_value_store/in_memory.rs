use super::KeyValueStore;
use async_trait::async_trait;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use wasmbus_rpc::actor::prelude::*;

pub struct InMemory {
    map: HashMap<String, String>,
    expiry: HashMap<String, Instant>,
}

impl InMemory {
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
                if time >= &Instant::now() {
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
        expires: u32,
    ) -> RpcResult<()> {
        let expiration = Instant::now() + Duration::from_secs(expires.into());
        self.map.insert(key.to_string(), value.to_string());
        self.expiry.insert(key.to_string(), expiration);

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
