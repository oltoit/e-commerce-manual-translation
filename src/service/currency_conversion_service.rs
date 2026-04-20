use std::collections::HashMap;
use reqwest::Client;
use serde::Deserialize;
use crate::config::env_loader::LOADER;
use crate::errors::error_enum::ErrorsEnum;

pub const SRC_CURRENCY: &str = "EUR";

pub async fn convert_currency_to_euro(currency: &str, amount: f64) -> Result<f64, ErrorsEnum> {
    let api_url = match LOADER.get() {
        Some(loader) => loader.get_fixer_address(),
        None => return Err(ErrorsEnum::EnvLoaderError("environment could not be retrieved".to_string()))
    };

    let response = match Client::new().get(api_url).send().await {
        Ok(response) => response,
        Err(_) => return Err(ErrorsEnum::FixerApiError)
    }.json::<FixerResponse>().await;

    match response {
        Ok(response) => {
            match response.rates.get(currency) {
                Some(rate) => Ok(
                    (amount * (1.0 / rate) * 100.0).round() / 100.0
                ),
                None => Err(ErrorsEnum::WrongCurrency(format!("currency '{}' not valid", currency)))
            }
        },
        Err(_) => Err(ErrorsEnum::JsonParsingError("error parsing fixer api response".to_string()))
    }
}

#[derive(Deserialize)]
struct FixerResponse {
    rates: HashMap<String, f64>,
}