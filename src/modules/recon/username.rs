use reqwest::{Client, StatusCode};

use crate::{config::ProviderCfg, core::error::FalconError};

/// Check a single provider for username presence.
pub async fn check_provider(
    client: &Client,
    provider: &ProviderCfg,
    username: &str,
)-> Result<bool, FalconError> {
    let url = provider.url.replace("{username}", username);
    let status = client.get(url).send().await?.status();
    Ok(matches!(
        status,
        StatusCode::OK
            | StatusCode::FOUND
            | StatusCode::MOVED_PERMANENTLY
            | StatusCode::SEE_OTHER
            | StatusCode::TEMPORARY_REDIRECT
            | StatusCode::PERMANENT_REDIRECT
    ))
}
