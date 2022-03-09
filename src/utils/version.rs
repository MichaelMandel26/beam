use anyhow::{Context, Result};
use reqwest::ClientBuilder;
use semver::Version;

pub async fn get_latest_release(url: &str) -> Result<Version> {
    let client = ClientBuilder::new().build()?;
    let response = client.head(url).send().await?;

    let version_string = response
        .url()
        .path_segments()
        .unwrap()
        .last()
        .ok_or_else(|| anyhow::anyhow!("Could not parse version string"))?;

    Version::parse(&version_string[1..]).context("Could not parse version string")
}

pub fn get_current_version() -> Version {
    Version::parse(env!("CARGO_PKG_VERSION")).unwrap()
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use httpmock::{prelude::*, Method};

    #[tokio::test]
    async fn test_get_latest_release() {
        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let github_latest_mock = server.mock(|when, then| {
            when.method(Method::HEAD)
                .path("/MichaelMandel26/beam/releases/latest");
            then.status(302).header(
                "Location",
                "https://github.com/MichaelMandel26/beam/releases/tag/v0.2.8",
            );
        });

        let latest_version =
            get_latest_release(&server.url("/MichaelMandel26/beam/releases/latest"))
                .await
                .unwrap();

        github_latest_mock.assert();
        assert_eq!(latest_version, Version::parse("0.2.8").unwrap());
    }

    #[test]
    fn test_get_current_version() {
        let current_version = get_current_version();
        assert_eq!(
            current_version,
            Version::parse(env!("CARGO_PKG_VERSION")).unwrap()
        );
    }
}
