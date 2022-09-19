use async_trait::async_trait;
use redis::{Commands, FromRedisValue};
use reqwest::get;

use crate::CACHEMANAGER;
pub struct TLDController;

#[async_trait]
pub trait TLDT {
    fn new() -> TLDController;

    async fn load(&mut self) -> &mut TLDController;
}

#[async_trait]
impl TLDT for TLDController {
    fn new() -> TLDController {
        TLDController 
    }

    async fn load(&mut self) -> &mut TLDController {
        let tld_count = CACHEMANAGER.lock()
            .await;

        let count: &str = tld_count.redis_instance.as_ref()
            .unwrap()
            .get::<&str, &str>("TLD:*")
            .unwrap();

        let response: String = get("https://data.iana.org/TLD/tlds-alpha-by-domain.txt")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        self
    }
}