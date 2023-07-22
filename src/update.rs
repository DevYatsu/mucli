const RELEASE_URL: &str = "https://github.com/DevYatsu/FileMorph/releases/latest";
use std::io::Error;

extern crate custom_error;
use custom_error::custom_error;

custom_error! {pub UpdateError
    Io{source: Error} = "{source}",
    ReqWest{source: reqwest::Error } = "{source}",
    GetLatest = "Failed to get latest release version",
    InvalidReleaseVersion = "No valid release version found",
    ErrorRetrievingVersion = "Cannot find release version"
}

pub async fn get_latest_release_version() -> Result<String, UpdateError> {
    let response = reqwest::get(RELEASE_URL).await?;

    let release_version = if let Some(segments) = response.url().path_segments() {
        let last_element = segments.last().unwrap();
        if last_element == "releases" {
            return Err(UpdateError::InvalidReleaseVersion);
        }
        last_element.to_string()
    } else {
        return Err(UpdateError::ErrorRetrievingVersion);
    };

    Ok(release_version)
}

pub fn can_update(current_v: &str, latest_v: &str) -> bool {
    let current_v = current_v
        .trim_start_matches("version")
        .trim_start_matches('v');
    let latest_v = latest_v
        .trim_start_matches("version")
        .trim_start_matches('v');

    current_v != latest_v
}
