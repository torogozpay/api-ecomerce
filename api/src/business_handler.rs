// api/src/test_handler.rs

use actix_web::{delete, get, post, put, web, HttpResponse};

use shared::error_handler::CustomError;
use application::business::{create, update, read, delete}; 
use domain::models::{Business,NewBusiness};


//use crate::utils::check;
use crate::utils::response;

#[utoipa::path(
    get,
    path = "/getBusiness",
    responses(
        (status = 200, description = "Get all businesses", body = inline(response::BusinessesResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[get("/getBusiness")]
pub async fn list_businesses_handler() -> Result<HttpResponse, CustomError> {
    // 👇 New function body!
    let businesses = web::block(read::list_businesses).await.unwrap();
    Ok(HttpResponse::Ok().json(businesses))
}

#[utoipa::path(
    get,
    path = "/getBusiness/{model_id}",
    responses(
        (status = 200, description = "Get a business identifies with id", body = inline(Business)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse)),
        (status = 404, description = "Business was not found", body = inline(response::ErrorResponse))
    )
)]
#[get("/getBusiness/{model_id}")]
pub async fn list_business_handler(model_id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    // 👇 New function body!
    let business = read::list_business(*model_id)?;
    Ok(HttpResponse::Ok().json(business))
}

#[utoipa::path(
    post,
    path = "/newBusiness",
    responses(
        (status = 200, description = "Create a new business", body = inline(response::BusinessResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[post("/newBusiness")]
pub async fn create_business_handler(business: web::Json<NewBusiness>) -> Result<HttpResponse, CustomError> {
    let business = create::create_business(business.into_inner())?;
    Ok(HttpResponse::Ok().json(business))
}

#[utoipa::path(
    put,
    path = "/updBusiness",
    responses(
    (status = 200, description = "Modify a new business", body = inline(response::BusinessResponse)),
    (status = 400, description = "Error", body = inline(response::ErrorResponse)),
    (status = 404, description = "Business was not found", body = inline(response::ErrorResponse))
    )
)]
#[put("/updBusiness")]
pub async fn update_business_handler(business: web::Json<Business>) -> Result<HttpResponse, CustomError> {
    let business = update::update_business(business.into_inner())?;
    Ok(HttpResponse::Ok().json(business))
}

#[utoipa::path(
    delete,
    path = "/delBusiness/{model_id}",
    responses(
        (status = 200, description = "Delete a new business", body = inline(response::BusinessResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse)),
        (status = 404, description = "Business was not found", body = inline(response::ErrorResponse))
    )
)]
#[delete("/delBusiness/{model_id}")]
pub async fn delete_business_handler(model_id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let businesses = delete::delete_business(*model_id)?;
    Ok(HttpResponse::Ok().json(businesses))
}