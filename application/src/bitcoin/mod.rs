// application/src/bitcoin/mod.rs
#![allow(non_snake_case)]

pub mod read;

use serde::{Deserialize, Serialize};
use domain::models::Currencies;
use shared::error_handler::CustomError;

use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
struct YadioResponse {
    BTC: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct BinanceResponse {
    price: String,
}


async fn get_btc_fiat_cash_price(currency: Currencies) -> Result<f64, CustomError> {
    //let body = reqwest::get("https://api.yadio.io/exrates/usd")
    let body = reqwest::get("https://api.yadio.io/exrates/".to_owned() + &currency.yadio)
        .await?
        .text();

	match body.await {
    	Ok(body) => {
   	        // Check the response body
            info!("Yadio Response: {:?}", body);
            // Deserialize JSON into struct
        	let yadio: Result<YadioResponse, serde_json::Error> = serde_json::from_str(&body);
            match yadio {
                Ok(yadio) => {
                    info!("Yadio Deserialized: {:?}", yadio);
                    Ok(yadio.BTC)    
                }
                Err(_) => Ok({
                    get_binance_price(currency).await?
                })
            }
        }
    	Err(_) => Ok({
            get_binance_price(currency).await?
    	})
	}

}

async fn get_binance_price(currency: Currencies) -> Result<f64, CustomError> {
    //let body = reqwest::get("https://api.binance.com/api/v3/avgPrice?symbol=BTCUSDT")
    let body2 = reqwest::get("https://api.binance.com/api/v3/avgPrice?symbol=".to_owned() + &currency.binance)
    .await?
    .text();

	match body2.await {
    	Ok(body2) => {    
            // Check the response body
            info!("Binance Response: {:?}", body2);

            // Deserialize JSON into struct
            let result2: Result<BinanceResponse, serde_json::Error> = serde_json::from_str(&body2);

            match result2 {
                Ok(binance) => {
                    info!("Binance Deserialized: {:?}", binance);
                    Ok(binance.price.parse().expect("Not a number!"))    
                }
                Err(_) => Err(CustomError::new(997, "Error deserialized".to_string()))
            }
        }
    	Err(_) => Err(CustomError::new(990, "Error converting".to_string()))
	}            
}     

pub async fn convert_currency_to_satoshi(currency: Currencies, amount: f64) -> Result<i64, CustomError> {
    let conversion_rate = get_btc_fiat_cash_price(currency.clone());
    //let conversion_rate = get_binance_price(currency.clone()).await?;

	match conversion_rate.await {
    	Ok(conversion_rate) => {
        	info!("Converting: {:?}", amount);
            let satoshi_amount = ((amount / conversion_rate) * 100_000_000.0).round() as i64;
            info!("{:.2} is equal to {:.8} (Satoshi)", amount, satoshi_amount);
            Ok(satoshi_amount.into())
    	}
    	Err(_) => {
            let conversion_rate = get_binance_price(currency.clone());

            match conversion_rate.await {
                Ok(conversion_rate) => {
                    info!("Converting: {:?}", amount);
                    let satoshi_amount = ((amount / conversion_rate) * 100_000_000.0).round() as i64;
                    info!("{:.2} is equal to {:.8} (Satoshi)", amount, satoshi_amount);
                    Ok(satoshi_amount.into())
                }
                Err(_) => Err(CustomError::new(990, "Error converting".to_string()))
            }    
        
    	}
	}

}