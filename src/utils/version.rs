use anyhow::{Context, Result};
use reqwest::ClientBuilder;
use semver::Version;

const LATEST_RELEASE_URL: &str = "https://github.com/MichaelMandel26/beam/releases/latest";

pub async fn get_latest_release() -> Result<Version> {
    let client = ClientBuilder::new().build()?;
    let response = client.head(LATEST_RELEASE_URL).send().await?;

    let version_string = response
        .url()
        .path_segments()
        .unwrap()
        .last()
        .ok_or(anyhow::anyhow!("Could not get latest release"))?;

    Version::parse(&version_string[1..]).context("Could not parse version string")
}

pub fn get_current_version() -> Version {
    Version::parse(env!("CARGO_PKG_VERSION")).unwrap()
}
