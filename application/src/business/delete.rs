// application/src/business/delete.rs
#![allow(non_snake_case)]

use crate::api_business_auth;
use domain::models::Business;
use domain::modelsext::{PreorderEmp,EmpMessage};
use infrastructure as db;
use diesel::prelude::*;
use shared::{settings::CONFIG, error_handler::CustomError};

use reqwest::{header, Client};
use tracing::{info, error};


pub async fn delete_business(model_id: i32) -> Result<Vec<Business>, CustomError> {
    use domain::schema::businesses::dsl::*;
    use domain::schema::businesses;

    let mut conn = db::connection()?;

    if model_id.clone() <= 0 {
        dbg!("Error, id is required and must not be empty");
        return Err(CustomError::new(400, "id is required and must not be empty".to_string()));
    } 
    
    let business = businesses::table.find(model_id).first::<Business>(&mut conn)?;

    match delete_business_in_api_business(business.clone()).await {
        Ok(busi) => {
            dbg!("Deleted business in ApiBusiness: {:?}", busi);

            diesel::delete(businesses.filter(id.eq(model_id))).execute(&mut conn)?;

            let mut business = businesses::table.select(businesses::all_columns).load::<Business>(&mut conn)?;
            business.sort();
            
            drop(conn);

            dbg!("List business in ApiEcommerce: {:?}", business.clone());

            Ok(business)  
        },    
        Err(e) => {
            dbg!("Error in ApiBusiness: {:?}", e.to_string());
            Err(CustomError::new(400, "Error saving business".to_string()))
        },    
    }
}

pub async fn delete_business_in_api_business(business: Business) -> Result<EmpMessage, CustomError> {
    let socket: String;
    socket = CONFIG.api.api_server.to_string();        

    match api_business_auth().await { 
        Ok(jwt) => {
            let emp_json = PreorderEmp {
                id: business.id,
                nombre: Some(business.app_name.clone()),
                descripcion: Some(business.app_name.clone()),
                activo: if business.enabled {1} else {0},
                apiId: Some(business.api_id),
                secretId: Some(business.api_secret),
                lnAddress: Some(business.ln_address)
            };  

            // Construct the request
            let client = Client::builder().build()?; 
            let response = client
                    .post(socket.to_owned() + "/api/v1/comercio/eliminar")
                    .header("Authorization", format!("Bearer {}", jwt))
                    .header(header::CONTENT_TYPE, "application/json") 
                    .json(&emp_json)
                    .send()
                    .await?;

            // Check the response body
            let body = response.text().await?;
            info!("Response Body EmpMessage: {:?}", body);
            
            // Deserialize JSON into struct
            let result: Result<EmpMessage, serde_json::Error> = serde_json::from_str(&body);

            match result {
                Ok(busi) => {
                    if busi.success {
                        Ok(busi)
                    } else {
                        error!("Error deleting business in ApiBusiness ...");
                        Err(CustomError::new(400, "Error saving business".to_string()))
                    }
                }
                Err(e) => {
                    error!("Error deserialized: {:?}", e);
                    Err(CustomError::new(400, e.to_string()))
                }
            }

        }
        Err(e) => {
            error!("Error Token: YES");
            Err(CustomError::new(400, e.to_string()))
        }  
    }
}    