use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

use tokio::sync::{Mutex, Semaphore};
use tokio::time::sleep;
use std::future::Future;

use crate::{
    config::AppConfig,
    core::error::FalconError,
    modules::recon::username,
};

#[derive(Debug, Clone)]
pub struct ReconResult {
    pub hits: usize,
    pub platforms: Vec<String>,
    pub emails: Vec<String>,
}

#[derive(Clone)]
pub struct Engine {
    client: reqwest::Client,
    cache: Arc<Mutex<HashMap<String, (Instant, ReconResult)>>>,
    semaphore: Arc<Semaphore>,
    cfg: Arc<AppConfig>,
}

impl Engine {
    pub fn new(cfg: AppConfig) -> Result<Self, FalconError> {
        let timeout = Duration::from_millis(cfg.timeout_ms);
        let client = reqwest::Client::builder()
            .user_agent("bloody-f4lcon/1.0 (research only)")
            .timeout(timeout)
            .redirect(reqwest::redirect::Policy::limited(4))
            .build()?;

        Ok(Self {
            client,
            cache: Arc::new(Mutex::new(HashMap::new())),
            semaphore: Arc::new(Semaphore::new(cfg.max_concurrent_scans)),
            cfg: Arc::new(cfg),
        })
    }

    pub async fn scan_username(
        &self,
        username: &str,
        use_cache: bool,
    ) -> Result<ReconResult, FalconError> {
        if use_cache {
            if let Some(cached) = self.check_cache(username).await {
                return Ok(cached);
            }
        }

        let mut hits = 0usize;
        let mut platforms = Vec::new();

        for provider in &self.cfg.providers {
            let permit = self.semaphore.acquire().await.map_err(|e| FalconError::Unknown(e.to_string()))?;
            let client = self.client.clone();
            let provider = provider.clone();
            let username = username.to_string();
            let res = with_backoff(|| async {
                username::check_provider(&client, &provider, &username).await
            })
            .await;
            drop(permit);

            match res {
                Ok(true) => {
                    hits += 1;
                    platforms.push(provider.name);
                }
                Ok(false) => {}
                Err(err) => {
                    tracing::warn!("provider {} error: {}", provider.name, err);
                }
            }
        }

        let result = ReconResult {
            hits,
            platforms,
            emails: Vec::new(),
        };

        if use_cache {
            self.store_cache(username.to_string(), result.clone()).await;
        }

        Ok(result)
    }

    async fn check_cache(&self, username: &str) -> Option<ReconResult> {
        let ttl = Duration::from_secs(self.cfg.cache_ttl_secs);
        let cache = self.cache.lock().await;
        cache.get(username).and_then(|(ts, v)| {
            if ts.elapsed() < ttl {
                Some(v.clone())
            } else {
                None
            }
        })
    }

    async fn store_cache(&self, username: String, value: ReconResult) {
        let mut cache = self.cache.lock().await;
        cache.insert(username, (Instant::now(), value));
    }
}

async fn with_backoff<F, Fut>(mut op: F) -> Result<bool, FalconError>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<bool, FalconError>>,
{
    let mut delay = Duration::from_millis(200);
    for attempt in 0..3 {
        match op().await {
            Ok(v) => return Ok(v),
            Err(err) => {
                if attempt == 2 {
                    return Err(err);
                }
                sleep(delay).await;
                delay *= 2;
            }
        }
    }
    Err(FalconError::Unknown("backoff exhausted".into()))
}
