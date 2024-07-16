#[allow(non_snake_case)]

pub mod bitcoin;
pub mod configuration;
pub mod business;
pub mod invoice;


use domain::models::{Business, User, NewUser};
use domain::modelsext::{CreateToken,Token};
use shared::{settings::CONFIG, error_handler::CustomError};
use infrastructure as db;
use diesel::prelude::*;
use actix_web::http::header::map::HeaderMap;

use reqwest::{header, Client};
use tracing::info;


pub async fn verificate_business(headers: &HeaderMap) -> Result<Business, CustomError> {
    let api_id: String = headers.get("api-id").unwrap().to_str().unwrap().to_string();
    let api_secret: String = headers.get("api-secret").unwrap().to_str().unwrap().to_string();
 
    let config = config_business(api_id, api_secret).await?;

    Ok(config)
 }
 
pub async fn config_business(api_id: String, api_secret: String) -> Result<Business, CustomError> {
    use domain::schema::businesses;

    let mut conn = db::connection()?;

    let business = businesses::table.filter(businesses::api_id.eq(api_id).and(businesses::api_secret.eq(api_secret))).select(Business::as_select()).get_result(&mut conn)?;
 
    Ok(business) 
}


pub async fn api_business_auth() -> Result<String, CustomError>{
    let username = CONFIG.api.api_user.to_string();
    let password = CONFIG.api.api_pass.to_string(); 
    
    let socket: String;
    socket = CONFIG.api.api_server.to_string();

    let data_user = CreateToken {
       username: username,
       password: password
    };
      
    // Construct the request
    let auth_client = Client::builder().build()?; 
    let token = auth_client
            .post(socket.to_owned() + "/api/v1/authenticate")
            .header(header::CONTENT_TYPE, "application/json") 
            .json(&data_user)
            .send()
            .await?;

    // Check the response body
    let token_str = token.text().await?;
    //info!("Response Token: {:?}", token_str);
    
    // Deserialize JSON into struct
    let token: Result<Token, serde_json::Error> = serde_json::from_str(&token_str);

    match token { 
        Ok(token) => {
            info!("Deserialized Token: {:?}", token);
            Ok(token.jwt)
        }
        Err(e) => {
            info!("Error deserialized Token: {:?}", e);
            Err(CustomError::new(700, e.to_string()))
        }  
    }
}   



pub async fn verificate_user(user: NewUser) -> Result<User, CustomError> {
    use domain::schema::users;

    let mut conn = db::connection()?;

    let user = users::table.filter(users::email.eq(user.email)).select(User::as_select()).get_result(&mut conn)?;
 
    Ok(user) 
}