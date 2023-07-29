use clap::ArgMatches;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use itertools::Itertools;
use qrcode::QrCode;
use wifi_qr_code::{AuthenticationType, Visibility, WifiCredentials};
use wifiscanner::{self};

use crate::print_err;

struct ReducedWifi {
    name: String,
    protocol: String,
}

pub fn qrcode_command(sub_matches: &ArgMatches) {
    let string = if let Some(string) = sub_matches.get_one::<String>("STRING") {
        string.to_string()
    } else {
        let wifi_credentials = if let Ok(w) = wifiscanner::scan() {
            let wifis = w
                .into_iter()
                .map(|w| ReducedWifi {
                    name: w.ssid,
                    protocol: w.security,
                })
                .unique_by(|w| w.name.to_string())
                .collect::<Vec<ReducedWifi>>();

            let choice = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select wifi name")
                .items(
                    &wifis
                        .iter()
                        .map(|w| w.name.to_string())
                        .collect::<Vec<String>>(),
                )
                .interact()
                .unwrap();

            let wifi_name = wifis[choice].name.to_string();

            WifiCredentials {
                ssid: wifi_name,
                authentication_type: protocol(&wifis[choice].protocol),
                visibility: Visibility::Visible,
            }
        } else {
            print_err!("Failed to scan available wifis. Please enter wifi name manually.");

            let wifi_name = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Wifi Name")
                .interact_text()
                .unwrap();
            let wifi_password = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Wifi password")
                .interact_text()
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

fn protocol(security: &str) -> AuthenticationType {
    match security {
        security if security.starts_with("WEP") => {
            let wifi_password = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Wifi password")
                .interact_text()
                .unwrap();
            AuthenticationType::WEP(wifi_password)
        }
        security if security.starts_with("WPA") || security.starts_with("RSN") => {
            let wifi_password = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Wifi password")
                .interact_text()
                .unwrap();
            AuthenticationType::WPA(wifi_password)
        }
        _ => AuthenticationType::NoPassword,
    }
}
