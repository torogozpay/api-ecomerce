// api/src/invoice_handler.rs

use actix_web::{delete, get, post, put, web, HttpResponse};

use shared::error_handler::CustomError;
use application::invoice::{create, update, read, delete};
use domain::models::{Invoice,MyInvoice,MyNewInvoice};

//use crate::utils::check;
use crate::utils::response;

#[utoipa::path(
    get,
    path = "/getInvoice",
    responses(
        (status = 200, description = "Get all invoices", body = inline(response::InvoicesResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[get("/getInvoice")]
pub async fn list_invoices_handler() -> Result<HttpResponse, CustomError> {
    let invoices= web::block(read::list_invoices).await.unwrap();
    Ok(HttpResponse::Ok().json(invoices))
}

#[utoipa::path(
    get,
    path = "/getInvoice/{model_id}",
    responses(
        (status = 200, description = "Get a invoice identifies with id", body = inline(Invoice)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse)),
        (status = 404, description = "Invoice was not found", body = inline(response::ErrorResponse))
    )
)]
#[get("/getInvoice/{model_id}")]
pub async fn list_invoice_handler(model_id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let invoices = read::list_invoice_by_id(*model_id).await?;
    Ok(HttpResponse::Ok().json(invoices))
}

#[utoipa::path(
    post,
    path = "/newInvoice",
    responses(
        (status = 200, description = "Create a new invoice", body = inline(response::MyInvoiceResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[post("/newInvoice")]
pub async fn create_invoice_handler(invoice: web::Json<MyNewInvoice>) -> Result<HttpResponse, CustomError> {  
    let invoices = create::start_payment(invoice.into_inner()).await?;
    Ok(HttpResponse::Ok().json(invoices))
}

#[utoipa::path(
    put,
    path = "/updInvoice",
    responses(
    (status = 200, description = "Modify a new invoice", body = inline(response::MyInvoiceResponse)),
    (status = 400, description = "Error", body = inline(response::ErrorResponse)),
    (status = 404, description = "Invoice was not found", body = inline(response::ErrorResponse))
    )
)]
#[put("/updInvoice")]
pub async fn update_invoice_handler(invoice: web::Json<MyInvoice>) -> Result<HttpResponse, CustomError> {
    let invoices = update::update_invoice(invoice.into_inner()).await?;
    Ok(HttpResponse::Ok().json(invoices))
}

#[utoipa::path(
    delete,
    path = "/delInvoice/{model_id}",
    responses(
        (status = 200, description = "Delete a new invoice", body = inline(response::InvoicesResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse)),
        (status = 404, description = "Invoice was not found", body = inline(response::ErrorResponse))
    )
)]
#[delete("/delInvoice/{model_id}")]
pub async fn delete_invoice_handler(model_id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let invoices = delete::delete_invoice(*model_id)?;
    Ok(HttpResponse::Ok().json(invoices))
}
