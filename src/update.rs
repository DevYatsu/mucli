use crate::{print_err, print_info, print_success, VERSION};
use custom_error::custom_error;
use dialoguer::{theme::ColorfulTheme, Confirm};
use std::io::Error;
custom_error! {pub UpdateError
    Io{source: Error} = "{source}",
    ReqWest{source: reqwest::Error } = "{source}",
    GetLatest = "Failed to get latest release version",
    InvalidReleaseVersion = "No valid release version found",
    ErrorRetrievingVersion = "Cannot find release version"
}

const RELEASE_URL: &str = "https://github.com/DevYatsu/mucli/releases/latest";

pub async fn update_command() {
    match get_latest_release_version().await {
        Ok(v) => {
            print_success!("Latest release version is \"{}\"", v);
            if can_update(VERSION, &v) {
                print_success!(
                    "This version is superior to current version \"{}\"",
                    VERSION
                );
                let confirmation = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Would you like to upgrade to the latest version?")
                    .interact()
                    .unwrap();

                if !confirmation {
                    return;
                }
                //update version
                print_info!("Feature coming soon!");
            }
        }
        Err(e) => print_err!("{}", e),
    };
}

async fn get_latest_release_version() -> Result<String, UpdateError> {
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

fn can_update(current_v: &str, latest_v: &str) -> bool {
    let current_v = current_v
        .trim()
        .trim_start_matches("version")
        .trim_start_matches('v');
    let latest_v = latest_v
        .trim()
        .trim_start_matches("version")
        .trim_start_matches('v');

    current_v != latest_v
}
