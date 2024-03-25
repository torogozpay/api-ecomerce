// application/src/bitcoin/create.rs

use domain::models::Currencies;
use domain::modelsext::CurrencyFilters;
use crate::bitcoin as priceBitcoin;
use infrastructure as db;
use diesel::prelude::*;
use shared::error_handler::CustomError;
use bigdecimal::ToPrimitive;


pub async fn get_satoshi(mydata: CurrencyFilters) -> Result<i64, CustomError> { 
    use domain::schema::currencies;

    let mut conn = db::connection()?;
    
    let curr = currencies::table.filter(currencies::currency.eq(mydata.currency.clone())).select(Currencies::as_select()).get_result(&mut conn)?;
    let amount = priceBitcoin::convert_currency_to_satoshi(curr, mydata.total_amount.to_f64().unwrap()).await?;

    Ok(amount.into())
}