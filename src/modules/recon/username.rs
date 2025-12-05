use reqwest::{Client, StatusCode};

use crate::{
    config::ProviderConfig,
    core::{engine::ProviderOutcome, error::FalconError},
};

/// Check a single provider for username presence.
pub async fn check_provider(
    client: &Client,
    provider: &ProviderConfig,
    username: &str,
) -> Result<ProviderOutcome, FalconError> {
    let url = provider.base_url.replace("{username}", username);
    let status = client.get(url).send().await?.status();
    if matches!(
        status,
        StatusCode::OK
            | StatusCode::FOUND
            | StatusCode::MOVED_PERMANENTLY
            | StatusCode::SEE_OTHER
            | StatusCode::TEMPORARY_REDIRECT
            | StatusCode::PERMANENT_REDIRECT
    ) {
        return Ok(ProviderOutcome::Hit);
    }
    if status == StatusCode::TOO_MANY_REQUESTS {
        return Ok(ProviderOutcome::RateLimited);
    }
    if status == StatusCode::UNAUTHORIZED || status == StatusCode::FORBIDDEN {
        return Ok(ProviderOutcome::Restricted);
    }
    Ok(ProviderOutcome::Miss)
}
