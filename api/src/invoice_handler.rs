// api/src/invoice_handler.rs

use actix_web::{post, web, HttpRequest, HttpResponse};
 
use shared::{authorization::verificate_token, error_handler::CustomError};
use application::invoice; 
use domain::modelsext::{NewInvoice,InvoiceFilters,OrderFilters};
use uuid::Uuid;

//use crate::utils::check;
use crate::utils::response as resp;
use tracing::info;

/// Get an order
#[utoipa::path(
    post,
    path = "/api/ecommerce/v1/getOrder",
    responses(
        (status = 200, description = "Get an order identifies with uuid", body = modelsext::InvoiceResp),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
        (status = 401, description = "Not authorizated", body = inline(resp::ErrorResponse))
    ),
    security(
        ("bearerAuth" = [])
    )
)]
#[post("/getOrder")]
pub async fn list_order_handler(myfilt: web::Json<OrderFilters>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    match verificate_token(req.headers()).await {
        Ok(_config) => {      

            let _new: Uuid;

            match Uuid::parse_str(&myfilt.uuid) {
                Ok(uuid) => {
                    _new = uuid;
                },
                Err(_) => {
                    _new = Default::default();
               }
            }

            let invoices = invoice::read::list_invoice_by_uuid(req.headers(), _new).await?;
            Ok(HttpResponse::Ok().json(invoices))

        },
        Err(_) => Err(CustomError::new(401, "Not authorizated".to_string()))
    }    
}

/// Get an invoice
#[utoipa::path(
    post,
    path = "/api/ecommerce/v1/getInvoice",
    responses(
        (status = 200, description = "Get an invoice identifies with payment_hash", body = modelsext::InvoiceResp),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
        (status = 401, description = "Not authorizated", body = inline(resp::ErrorResponse))
    ),
    security(
        ("bearerAuth" = [])
    )
)]
#[post("/getInvoice")]
pub async fn list_invoice_handler(myfilt: web::Json<InvoiceFilters>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    match verificate_token(req.headers()).await {
        Ok(_config) => {      
            let invoices = invoice::read::list_invoice_by_hash(req.headers(), myfilt.hash.clone()).await?;
            Ok(HttpResponse::Ok().json(invoices))
        },
        Err(_) => Err(CustomError::new(401, "Not authorizated".to_string()))
    }    
}

/// Save an order
#[utoipa::path(
    post,
    path = "/api/ecommerce/v1/savePreorder",
    responses(
        (status = 200, description = "Create a new pre order", body = modelsext::InvoiceResp),
        (status = 400, description = "Error", body = inline(resp::ErrorResponse)),
        (status = 401, description = "Not authorizated", body = inline(resp::ErrorResponse))
    ),
    security(
        ("bearerAuth" = [])
    )
)]
#[post("/savePreorder")]
pub async fn save_preorder_handler(myinvoice: web::Json<NewInvoice>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    
    info!("Headers= {:?}", req.headers());
    info!("JsonWooCommerce= {:#?}", myinvoice.clone());

    match verificate_token(req.headers()).await {
        Ok(business_id) => {      
            let businesses = invoice::create::save_pre_order(req.headers(), business_id, myinvoice.into_inner()).await?;
            Ok(HttpResponse::Ok().json(businesses))
        },
        Err(_) => Err(CustomError::new(401, "Not authorizated".to_string()))
    }    
}