use std::time::Duration;

use anyhow::Result;
use reqwest::{Client, StatusCode};

/// Outcome of a username scan across platforms.
#[derive(Debug, Clone)]
pub struct ScanOutcome {
    pub hits: usize,
    pub platforms: Vec<String>,
    pub emails: Vec<String>,
}

/// Scan a username across a curated list of public platforms using HTTP presence checks.
pub async fn scan_username(username: String) -> Result<ScanOutcome> {
    let client = Client::builder()
        .user_agent("bloody-falcon/1.0 (research only)")
        .timeout(Duration::from_secs(4))
        .redirect(reqwest::redirect::Policy::limited(4))
        .build()?;

    let targets = vec![
        ("GitHub", format!("https://github.com/{username}")),
        ("Reddit", format!("https://www.reddit.com/user/{username}")),
        ("Steam", format!("https://steamcommunity.com/id/{username}")),
        ("Twitter", format!("https://twitter.com/{username}")),
        ("PSNProfiles", format!("https://psnprofiles.com/{username}")),
    ];

    let mut tasks = Vec::with_capacity(targets.len());
    for (name, url) in targets {
        let client_clone = client.clone();
        tasks.push(tokio::spawn(async move {
            let ok = check_presence(&client_clone, &url).await.unwrap_or(false);
            ok.then(|| name.to_string())
        }));
    }

    let mut live = Vec::new();
    for task in tasks {
        match task.await {
            Ok(Some(name)) => live.push(name),
            _ => {}
        };
    }

    Ok(ScanOutcome {
        hits: live.len(),
        platforms: live,
        emails: Vec::new(), // real email discovery requires dedicated modules; kept empty here
    })
}

async fn check_presence(client: &Client, url: &str) -> Result<bool> {
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
