// application/src/test/create.rs

use domain::models::{Business, NewBusiness};
use domain::modelsext::BusinessCreated;
use infrastructure as db;
use diesel::prelude::*;
use shared::error_handler::CustomError;
use shared::generate_numbers::{gen_api_id, gen_api_secret};

pub fn create_business(business: NewBusiness) -> Result<Business, CustomError> {
    use domain::schema::businesses;

    let mut conn = db::connection()?;

    let id = gen_api_id();
    let secret = gen_api_secret();

    let mut mybusiness = business;
    mybusiness.api_id = id;
    mybusiness.api_secret = secret;

    let business = NewBusiness::from(mybusiness);

    let business = diesel::insert_into(businesses::table).values(&business).get_result::<Business>(&mut conn)?; 
    Ok(business)    
}

pub fn generate_data_api() -> Result<BusinessCreated, CustomError> {

    let id = gen_api_id();
    let secret = gen_api_secret();

    let data_api = BusinessCreated {
        api_id: id,
        api_secret: secret 
    };

    Ok(data_api)
}