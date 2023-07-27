use clap::ArgMatches;
use dialoguer::{theme::ColorfulTheme, Input, Password, Select};
use itertools::Itertools;
use qrcode::QrCode;
use wifi_qr_code::{AuthenticationType, Visibility, WifiCredentials};
use wifiscanner;

use crate::print_err;

pub fn qrcode_command(sub_matches: &ArgMatches) {
    let string = if let Some(string) = sub_matches.get_one::<String>("STRING") {
        string.to_string()
    } else {
        let wifi_credentials = if let Ok(w) = wifiscanner::scan() {
            let wifis = w
                .into_iter()
                .map(|w| w.ssid)
                .unique()
                .collect::<Vec<String>>();

            let choice = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select wifi name")
                .items(&wifis)
                .interact()
                .unwrap();

            let wifi_password = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Wifi password")
                .interact_text()
                .unwrap();

            WifiCredentials {
                ssid: wifis[choice].to_string(),
                authentication_type: AuthenticationType::WPA(wifi_password),
                visibility: Visibility::Visible,
            }
        } else {
            print_err!("Failed to scan available wifis. Please enter wifi name manually.");

            let wifi_name = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Wifi Name")
                .interact_text()
                .unwrap();
            let wifi_password = Password::with_theme(&ColorfulTheme::default())
                .with_prompt("Wifi password")
                .interact()
                .unwrap();

            WifiCredentials {
                ssid: wifi_name,
                authentication_type: AuthenticationType::WPA(wifi_password),
                visibility: Visibility::Visible,
            }
        };

        wifi_credentials.encode()
    };

    let code = QrCode::new(format!("{string}").as_bytes()).unwrap();
    let string = code
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();
    print!("{}", string);
}
