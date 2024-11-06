use application::bitcoin;
use actix_web::{web, post, get, HttpRequest, HttpResponse};

use shared::{error_handler::CustomError, authorization::verificate_token};
use domain::modelsext::CurrencyFilters;
use crate::utils::response as resp;


#[utoipa::path(
    get,
    path = "/api/ecommerce/v1/health_check",
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

/// Exchange total amount from USD to Satoshis
#[utoipa::path(
    post,
    path = "/api/ecommerce/v1/getSatoshi",
    responses(
        (status = 200, description = "Get Bitcoin exchange rate info"),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
    ),
    security(
        ("bearerAuth" = [])
    )
)]
#[post("/getSatoshi")]
pub async fn get_price_bitcoin_handler(mydata : web::Json<CurrencyFilters>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    match verificate_token(req.headers()).await {
        Ok(_config) => {

            let sats = bitcoin::read::get_satoshi(mydata.into_inner())                
                .await
                .unwrap();

            Ok(HttpResponse::Ok().json(sats))
        },
        Err(_) => Err(CustomError::new(401, "Not authorized".to_string())),
    }
     
}