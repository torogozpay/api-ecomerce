// api/src/business_handler.rs

use actix_web::{delete, get, post, put, web, HttpResponse};
 
use shared::error_handler::CustomError;
use application::business::{create, update, read, delete}; 
use application::verificate_user;
use domain::models::{Business,NewBusiness,NewUser};
use domain::modelsext::BusinessCreated;

//use crate::utils::check;
use crate::utils::response;

/// Get all businesses
#[utoipa::path(
    get,
    path = "/api/ecommerce/v1/getBusiness",
    responses(
        (status = 200, description = "Get all businesses", body = response::BusinessesResponse),
        (status = 400, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[get("/getBusiness")]
pub async fn list_businesses_handler() -> Result<HttpResponse, CustomError> {
    let businesses = web::block(read::list_businesses).await.unwrap();
    Ok(HttpResponse::Ok().json(businesses))
}

/// Get a business by id
#[utoipa::path(
    get,
    path = "/api/ecommerce/v1/getBusiness/{model_id}",
    responses(
        (status = 200, description = "Get a business identifies with id", body = Business),
        (status = 400, description = "Error", body = inline(response::ErrorResponse)),
        (status = 404, description = "Business was not found", body = inline(response::ErrorResponse))
    )
)]
#[get("/getBusiness/{model_id}")]
pub async fn list_business_by_id_handler(model_id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let business = read::list_business(*model_id)?;
    Ok(HttpResponse::Ok().json(business))
}

/// Get a business by API data
#[utoipa::path(
    post,
    path = "/api/ecommerce/v1/getBusiness",
    responses(
        (status = 200, description = "Get a business identifies with api data", body = Business),
        (status = 400, description = "Error", body = inline(response::ErrorResponse)),
        (status = 404, description = "Business was not found", body = inline(response::ErrorResponse))
    )
)]
#[post("/getBusiness")]
pub async fn list_business_by_api_handler(business: web::Json<BusinessCreated>) -> Result<HttpResponse, CustomError> {
    let business = read::get_business(business.into_inner())?;
    Ok(HttpResponse::Ok().json(business))
}

/// Create a business
#[utoipa::path(
    post,
    path = "/api/ecommerce/v1/newBusiness",
    responses(
        (status = 200, description = "Create a new business", body = response::BusinessResponse),
        (status = 400, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[post("/newBusiness")]
pub async fn create_business_handler(business: web::Json<NewBusiness>) -> Result<HttpResponse, CustomError> {
    let business = create::create_business(business.into_inner()).await?;
    Ok(HttpResponse::Ok().json(business))
}

/// Update a business
#[utoipa::path(
    put,
    path = "/api/ecommerce/v1/updBusiness",
    responses(
    (status = 200, description = "Modify a new business", body = response::BusinessResponse),
    (status = 400, description = "Error", body = inline(response::ErrorResponse)),
    (status = 404, description = "Business was not found", body = inline(response::ErrorResponse))
    )
)]
#[put("/updBusiness")]
pub async fn update_business_handler(business: web::Json<Business>) -> Result<HttpResponse, CustomError> {
    let business = update::update_business(business.into_inner()).await?;
    Ok(HttpResponse::Ok().json(business))
}

/// Delete a business
#[utoipa::path(
    delete,
    path = "/api/ecommerce/v1/delBusiness/{model_id}",
    responses(
        (status = 200, description = "Delete a new business", body = response::BusinessResponse),
        (status = 400, description = "Error", body = inline(response::ErrorResponse)),
        (status = 404, description = "Business was not found", body = inline(response::ErrorResponse))
    )
)]
#[delete("/delBusiness/{model_id}")]
pub async fn delete_business_handler(model_id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let businesses = delete::delete_business(*model_id).await?;
    Ok(HttpResponse::Ok().json(businesses))
}

/// Generate API data
#[utoipa::path(
    post,
    path = "/api/ecommerce/v1/newBusinessApi",
    responses(
        (status = 200, description = "Generate data api", body = response::BusinessResponse),
        (status = 400, description = "Error", body = inline(response::ErrorResponse)),
        (status = 401, description = "Not authorizated", body = inline(response::ErrorResponse))
    )
)]
#[post("/newBusinessApi")]
pub async fn generate_data_api_handler(user: web::Json<NewUser>) -> Result<HttpResponse, CustomError> {
    match verificate_user(user.into_inner()).await {
        Ok(_config) => {
            let data_api = create::generate_data_api()?;
            Ok(HttpResponse::Ok().json(data_api))
        },
        Err(_) => Err(CustomError::new(401, "Not authorizated".to_string())),
    }
}