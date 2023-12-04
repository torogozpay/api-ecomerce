// apllication/src/invoice/create.rs

use lazy_static::lazy_static;

use domain::models::{BusinessNode, Invoice, MyNewInvoice, MyInvoice, InvoiceDet, NewInvoice, NewInvoiceDet};
use domain::modelsext::{InvoiceCreated, InvoiceResponse};
use infrastructure as db;
use diesel::prelude::*;
use shared::error_handler::CustomError;
use reqwest::{header, Client};
use bigdecimal::{BigDecimal, ToPrimitive};

use shared::settings;

lazy_static! {
    static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("Config can be loaded");
}


pub async fn start_payment(_config: BusinessNode, myinvoice : InvoiceCreated) -> Result<InvoiceResponse, CustomError> {
   let _invoice_node = generate_invoice(&_config, &myinvoice.amount, &myinvoice.description).await?;

   Ok(_invoice_node)
}


pub async fn generate_invoice(config : &BusinessNode, amount : &BigDecimal, description : &String) -> Result<InvoiceResponse, CustomError> {
    let api_key = CONFIG.api.api_key.clone();
    let username = CONFIG.api.api_username.clone();
    let password = CONFIG.api.api_password.clone();
    let auth_string = format!("{}:{}", username, password);
    let auth_encoded = "Basic ".to_owned() + &base64::encode(&auth_string);
   
    let socket: String;
    socket = config.host.to_string() + ":" + &config.port.to_string();
    
    let json = &serde_json::json!({
        "lnd": config.lnd,
        "socket": socket.to_string(),
        "macaroon": config.macaroon.to_string(),
        "cert": config.cert.to_string(),
        "path": config.path.to_string(),
        "expiry": config.expiry as u32,
        "cltv": config.cltv as u32,
        "description": description.to_string(),
        "amount": amount.to_u64().unwrap()
    });
     
    // Construct the request
    let client = Client::builder().build()?; 
    let response = client
            .post("https://a085-138-186-251-29.ngrok-free.app/api/v1/createInvoice")
            .header("Authorization", format!("{}", auth_encoded))
            .header("x-api-key", api_key) 
            .header(header::CONTENT_TYPE, "application/json") 
            .json(&json)
            .send()
            .await?;

    // Check the response body
    let body = response.text().await?;
    println!("Response Body: {:?}", body);    
    
    // Deserialize JSON into struct
    let result: Result<InvoiceResponse, serde_json::Error> = serde_json::from_str(&body);

    match result {
        Ok(your_struct) => {
            println!("Deserialized struct: {:?}", your_struct);
            Ok(your_struct)    
        }
        Err(e) => {
            println!("Error deserialized: {:?}", e);
            Err(CustomError::new(997, e.to_string()))
        }
    }
         
}    

pub async fn send_payment(_config: BusinessNode, mut myinvoice: MyNewInvoice) -> Result<MyInvoice, CustomError> {    
    myinvoice.master.business_id = _config.business_id.clone();    
    myinvoice.master.payment_status = Some("paid".to_owned());
    let invoice = create_invoice(myinvoice).await?;

    Ok(invoice)
}

pub async fn create_invoice(myinvoice: MyNewInvoice) -> Result<MyInvoice, CustomError> { 
    use domain::schema::invoices;
    use domain::schema::invoices_det;

    let mut conn = db::connection()?;

    let invoice = NewInvoice::from(myinvoice.master);

    let newinvoice = diesel::insert_into(invoices::table).values(&invoice).get_result::<Invoice>(&mut conn)?;  

    let mut invoicedets: Vec<NewInvoiceDet> = Vec::new();
    for mut element in myinvoice.details {
        element.invoice_id = newinvoice.id;
        let row = NewInvoiceDet::from(element);
        invoicedets.push(row);
    } 

    let newinvoicedets = diesel::insert_into(invoices_det::table).values(&invoicedets).get_results::<InvoiceDet>(&mut conn)?;

    let myresult = MyInvoice { master: (newinvoice), details: (newinvoicedets) };
    Ok(myresult)
}