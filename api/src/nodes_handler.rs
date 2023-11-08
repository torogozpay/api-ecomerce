// api/src/test_handler.rs

use actix_web::{delete, get, post, put, web, HttpResponse};

use shared::error_handler::CustomError;
use application::nodes::{create, update, read, delete}; 
use domain::models::{BusinessNode,NewBusinessNode};


//use crate::utils::check;
use crate::utils::response;

#[utoipa::path(
    get,
    path = "/getBusinessNode",
    responses(
        (status = 200, description = "Get all businesses nodes", body = inline(response::BusinessNodesResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[get("/getBusinessNode")]
pub async fn list_businesses_nodes_handler() -> Result<HttpResponse, CustomError> {
    // 👇 New function body!
    let businesses = web::block(read::list_businesses_nodes).await.unwrap();
    Ok(HttpResponse::Ok().json(businesses))
}

#[utoipa::path(
    get,
    path = "/getBusinessNode/{model_id}",
    responses(
        (status = 200, description = "Get a business node identifies with id", body = inline(BusinessNode)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse)),
        (status = 404, description = "Business was not found", body = inline(response::ErrorResponse))
    )
)]
#[get("/getBusinessNode/{model_id}")]
pub async fn list_business_node_handler(model_id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    // 👇 New function body!
    let business = read::list_business_node(*model_id)?;
    Ok(HttpResponse::Ok().json(business))
}

#[utoipa::path(
    post,
    path = "/newBusinessNode",
    responses(
        (status = 200, description = "Create a new business", body = inline(response::BusinessNodeResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[post("/newBusinessNode")]
pub async fn create_business_node_handler(business: web::Json<NewBusinessNode>) -> Result<HttpResponse, CustomError> {
    let business = create::create_business_node(business.into_inner())?;
    Ok(HttpResponse::Ok().json(business))
}

#[utoipa::path(
    put,
    path = "/updBusinessNode",
    responses(
    (status = 200, description = "Modify a new business", body = inline(response::BusinessNodeResponse)),
    (status = 400, description = "Error", body = inline(response::ErrorResponse)),
    (status = 404, description = "Business was not found", body = inline(response::ErrorResponse))
    )
)]
#[put("/updBusinessNode")]
pub async fn update_business_node_handler(business: web::Json<BusinessNode>) -> Result<HttpResponse, CustomError> {
    let business = update::update_business_node(business.into_inner())?;
    Ok(HttpResponse::Ok().json(business))
}

#[utoipa::path(
    delete,
    path = "/delBusinessNode/{model_id}",
    responses(
        (status = 200, description = "Delete a new business", body = inline(response::BusinessNodeResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse)),
        (status = 404, description = "Business was not found", body = inline(response::ErrorResponse))
    )
)]
#[delete("/delBusinessNode/{model_id}")]
pub async fn delete_business_node_handler(model_id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let businesses = delete::delete_business_node(*model_id)?;
    Ok(HttpResponse::Ok().json(businesses))
}