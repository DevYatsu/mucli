use std::collections::HashMap;

use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Select};
use dotenv_codegen::dotenv;
use serde::{Deserialize, Serialize};

use crate::{print_err, print_success, utils::GenericError};

const SUPPORTED_CURRENCIES: [&str; 33] = [
    "EUR", "USD", "JPY", "BGN", "CZK", "DKK", "GBP", "HUF", "PLN", "RON", "SEK", "CHF", "ISK",
    "NOK", "HRK", "RUB", "TRY", "AUD", "BRL", "CAD", "CNY", "HKD", "IDR", "ILS", "INR", "KRW",
    "MXN", "MYR", "NZD", "PHP", "SGD", "THB", "ZAR",
];

pub async fn currency_command() {
    let input_curr = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Source currency")
        .items(&SUPPORTED_CURRENCIES)
        .interact()
        .unwrap();

    let quantity = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Quantity of {}", SUPPORTED_CURRENCIES[input_curr]))
        .validate_with(|input: &String| -> Result<(), &str> {
            if let Ok(_) = input.parse::<f64>() {
                Ok(())
            } else {
                Err("Value must be a number!")
            }
        })
        .interact_text()
        .unwrap();

    let output_curr = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select all output currencies")
        .items(&SUPPORTED_CURRENCIES)
        .interact()
        .unwrap();

    let selected_output_curr = output_curr
        .into_iter()
        .map(|i| SUPPORTED_CURRENCIES[i])
        .collect::<Vec<&str>>();

    let response =
        match get_exchange_rates(SUPPORTED_CURRENCIES[input_curr], selected_output_curr).await {
            Ok(r) => r,
            Err(e) => {
                print_err!("{}", e);
                return;
            }
        };

    print_success!(
        "{} {} corresponds to:",
        quantity,
        SUPPORTED_CURRENCIES[input_curr]
    );
    for (curr, rate) in response.data {
        println!("{:.2} {}", quantity.parse::<f64>().unwrap() * rate, curr);
    }
}

async fn get_exchange_rates(base_curr: &str, output: Vec<&str>) -> Result<Response, GenericError> {
    const API_KEY: &str = dotenv!("CURRENCY_API_KEY");
    let output_curr = output.join("%2C");
    let url = format!("https://api.freecurrencyapi.com/v1/latest?apikey={API_KEY}&currencies={output_curr}&base_currency={base_curr}");

    let text = reqwest::get(url).await?.text().await?;

    Ok(serde_json::from_str(&text)?)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Response {
    meta: Option<ResponseMeta>,
    data: HashMap<String, f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ResponseMeta {
    last_updated_at: String,
}
