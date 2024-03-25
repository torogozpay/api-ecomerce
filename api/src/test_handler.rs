use application::{bitcoin, verificate_business};
use actix_web::{web, get, HttpRequest, HttpResponse};

use shared::error_handler::CustomError;
use domain::modelsext::CurrencyFilters;
use crate::utils::response as resp;


#[utoipa::path(
    get,
    path = "/health_check",
    responses(
        (status = 200, description = "Testing"),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
    )
)]
#[get("/health_check")]
pub async fn get_test_handler() -> Result<HttpResponse, CustomError> {

    let info = "API Ecommerce started";
 
    Ok(HttpResponse::Ok().json(info))        
}

#[utoipa::path(
    get,
    path = "/getSatoshi",
    responses(
        (status = 200, description = "Get info of bitcoin"),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
    )
)]
#[get("/getSatoshi")]
pub async fn get_price_bitcoin_handler(mydata : web::Json<CurrencyFilters>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    match verificate_business(req.headers()).await {
        Ok(_config) => {

            let sats = bitcoin::read::get_satoshi(mydata.into_inner())                
                .await
                .unwrap();

            Ok(HttpResponse::Ok().json(sats))
        },
        Err(_) => Err(CustomError::new(401, "Not authorizated".to_string())),
    }
     
}