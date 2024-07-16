// application/src/business/create.rs
#![allow(non_snake_case)]

use crate::api_business_auth;
use domain::models::{Business, NewBusiness};
use domain::modelsext::{BusinessCreated, PreorderEmp, EmpMessage};
use infrastructure as db;
use diesel::prelude::*;
use shared::{settings::CONFIG, error_handler::CustomError};
use shared::generate_numbers::{gen_api_id, gen_api_secret};
use uuid::Uuid;

use reqwest::{header, Client};
use tracing::{info, error};

pub fn generate_data_api() -> Result<BusinessCreated, CustomError> {

    let id = gen_api_id();
    let secret = gen_api_secret();

    let data_api = BusinessCreated {
        api_id: id,
        api_secret: secret 
    };

    Ok(data_api)
}

pub async fn create_business(business: NewBusiness) -> Result<Business, CustomError> {
    use domain::schema::businesses;

    let mut conn = db::connection()?;

    let mut mybusiness = business.clone();

    if business.api_id.clone().trim().len() == 0 || business.api_secret.clone().trim().len() == 0 {
        mybusiness.api_id = gen_api_id();
        mybusiness.api_secret = gen_api_secret();
    } 

    // Check the presence of app_name
    if mybusiness.app_name.clone().trim().len() == 0 {
        error!("Error, app_name is required and must not be empty");
        return Err(CustomError::new(400, "app_name is required and must not be empty".to_string()));
    }
    // Check the presence of app_logo
    if mybusiness.app_logo.clone().trim().len() == 0 {
        error!("Error, app_logo is required and must not be empty");
        return Err(CustomError::new(400, "app_logo is required and must not be empty".to_string()));
    }
    // Check the presence of app_url
    if mybusiness.app_url.clone().trim().len() == 0 {
        error!("Error, app_url is required and must not be empty");
        return Err(CustomError::new(400, "app_url is required and must not be empty".to_string()));
    }        
    // Check the presence of workspace_id
    let uuid_v4 = mybusiness.workspace_id.clone();
    if uuid_v4.to_string().trim().len() == 0 {
        mybusiness.workspace_id = Uuid::new_v4();
    } 
    // Check the presence of ln_address
    if mybusiness.ln_address.clone().trim().len() == 0 {
        error!("Error, ln_address is required and must not be empty");
        return Err(CustomError::new(400, "ln_address is required and must not be empty".to_string()));
    }

    let business = NewBusiness::from(mybusiness);

    let business = diesel::insert_into(businesses::table).values(&business).get_result::<Business>(&mut conn)?; 

    drop(conn);

    match create_business_in_api_business(business.clone()).await {
        Ok(busi) => {
            info!("Created business in ApiEcommerce: {:?}", business);
            info!("Created business in ApiBusiness: {:?}", busi);
            Ok(business)  
        },    
        Err(e) => {
            error!("Error in ApiBusiness: {:?}", e.to_string());
            Err(CustomError::new(400, "Error saving business".to_string()))
        },    
    }
      
}



pub async fn create_business_in_api_business(business: Business) -> Result<EmpMessage, CustomError> {
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
                    .post(socket.to_owned() + "/api/v1/comercio/agregar")
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
                        error!("Error saving business in ApiBusiness ...");
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