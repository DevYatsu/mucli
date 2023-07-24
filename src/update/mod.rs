mod annex;

use dialoguer::{theme::ColorfulTheme, Confirm};

use crate::{print_err, print_success, VERSION, print_future_update};

use self::annex::{can_update, get_latest_release_version};

pub async fn update_command() {
    match get_latest_release_version().await {
        Ok(v) => {
            print_success!("Latest release version is \"{}\"", v);
            if can_update(VERSION, &v) {
                print_success!("This version is superior to current version \"{}\"", VERSION);
                let confirmation = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Would you like to upgrade to the latest version?")
                    .interact()
                    .unwrap();

                if !confirmation {
                    return;
                }
                //update version
                print_future_update!("Feature coming soon!");
            }
        }
        Err(e) => print_err!("{}", e),
    };
}
