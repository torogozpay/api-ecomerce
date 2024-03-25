// api/src/invoice_handler.rs

use actix_web::{get, post, web, HttpRequest, HttpResponse};

use shared::{authorization::verificate_token, error_handler::CustomError};
use application::invoice; 
use domain::modelsext::{NewInvoice,InvoiceResponse,InvoiceFilters};

//use crate::utils::check;
use crate::utils::response;
use tracing::info;

#[utoipa::path(
    get,
    path = "/getInvoice",
    responses(
        (status = 200, description = "Get a invoice identifies with payment_hash", body = inline(InvoiceResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse)),
        (status = 404, description = "Invoice was not found", body = inline(response::ErrorResponse))
    )
)]
#[get("/getInvoice")]
pub async fn list_invoice_handler(myfilt: web::Json<InvoiceFilters>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    match verificate_token(req.headers()).await {
        Ok(_config) => {      
            let invoices = invoice::read::list_invoice_by_hash(myfilt.hash.clone()).await?;
            Ok(HttpResponse::Ok().json(invoices))
        },
        Err(_) => Err(CustomError::new(401, "Not authorizated".to_string()))
    }    
}

#[utoipa::path(
    post,
    path = "/savePreorder",
    responses(
        (status = 200, description = "Create a new pre order", body = inline(InvoiceResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse)),
        (status = 404, description = "Pre order was not found", body = inline(response::ErrorResponse))
    )
)]
#[post("/savePreorder")]
pub async fn save_preorder_handler(myinvoice: web::Json<NewInvoice>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    
    info!("Headers= {:?}", req.headers());
    info!("JsonWooCommerce= {:#?}", myinvoice.clone());

    match verificate_token(req.headers()).await {
        Ok(business_id) => {      
            let businesses = invoice::create::save_pre_order(business_id, myinvoice.into_inner()).await?;
            Ok(HttpResponse::Ok().json(businesses))
        },
        Err(_) => Err(CustomError::new(401, "Not authorizated".to_string()))
    }    
}