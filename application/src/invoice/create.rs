// application/src/invoice/create.rs
#![allow(non_snake_case)]

use domain::models::Currencies;
use domain::modelsext::{DataInvoice, PreorderSplit, NewInvoice, NewInvoiceDet, InvoiceData, Preorder};
use crate::business;
use crate::configuration::read;
use crate::api_business_auth;
use infrastructure as db;
use diesel::prelude::*;
use shared::{settings::CONFIG, error_handler::CustomError};
use reqwest::{header, Client};
use actix_web::http::header::map::HeaderMap;

use crate::bitcoin as priceBitcoin;
use bigdecimal::{BigDecimal,ToPrimitive,Zero};
use uuid::Uuid;

use tracing::{info, error};


pub async fn save_pre_order(headers: &HeaderMap, business_id: i32, mut preventa: NewInvoice) -> Result<DataInvoice, CustomError> { 
    use domain::schema::currencies;
    let mut conn = db::connection()?;

    let config = read::list_config(); 
    let mybusiness = business::read::list_business(business_id);

    preventa.businessId = business_id; 
    preventa.applySplit = mybusiness.unwrap().apply_split; 

    // Check the presence of amount_min
    let amount_min: BigDecimal = config.unwrap().amount_min;
    if amount_min <= BigDecimal::zero() {
        error!("Error, amount_min is required");
        return Err(CustomError::new(400, "amount_min is required".to_string()));
    } 

    // Check the presence of business_id
    if preventa.businessId <= 0 {
       error!("Error, business_id is required");
       return Err(CustomError::new(400, "business_id is required".to_string()));
    }
    // Check the presence of order_id
    if preventa.orderId <= 0 {
        error!("Error, order_id is required");
        return Err(CustomError::new(400, "order_id is required".to_string()));
    }

    // Check the presence of customer_name
    if preventa.customerName.trim().len() == 0 {
       error!("Error, customer_name is required and must not be empty");
       return Err(CustomError::new(400, "customer_name is required and must not be empty".to_string()));
    }

    // Check the presence of customer_email
    if preventa.customerEmail.trim().len() == 0 {
       error!("Error, customer_email is required and must not be empty");
       return Err(CustomError::new(400, "customer_email is required and must not be empty".to_string()));
    }

    // Check the presence of currency
    if preventa.currency.trim().len() == 0 {
       error!("Error, currency is required and must not be empty");
       return Err(CustomError::new(400, "currency is required and must not be empty".to_string()));
    }

    // Check the presence of sub_total
    if preventa.subTotal <= BigDecimal::zero() {
       error!("Error, sub_total is required and must not be zero");
       return Err(CustomError::new(400, "sub_total is required and must not be zero".to_string()));
    }

    // Check the presence of total_amount
    if preventa.totalAmount <= BigDecimal::zero() {
       error!("Error, total_amount is required and must not be zero");
       return Err(CustomError::new(400, "total_amount is required and must not be zero".to_string()));
    }

    // Check the presence of invoice_stamp
    if preventa.invoiceStamp.trim().len() == 0  {
       error!("Error, invoice_stamp is required");
       return Err(CustomError::new(400, "invoice_stamp is required".to_string()));
    }

    // Check the presence of details
    if !preventa.details.clone().iter().all(|x| is_valid_row_details(&x, amount_min.clone())) {
        error!("Error, there are inconsistency in the details");
        return Err(CustomError::new(400, "There are inconsistency in the details".to_string()));
    }

    // Converting to satoshis
    
    let curr = currencies::table.filter(currencies::currency.eq(preventa.currency.clone())).select(Currencies::as_select()).get_result(&mut conn)?;
    let amount = priceBitcoin::convert_currency_to_satoshi(curr.clone(), preventa.totalAmount.to_f64().unwrap());
    drop(conn);

    match amount.await {
        Ok(amountTotal) => {
            preventa.totalSats = amountTotal;

            for det in &mut preventa.details {
                match priceBitcoin::convert_currency_to_satoshi(curr.clone(), det.grandTotal.to_f64().expect("Error converting")).await {
                    Ok(amount) => {
                        det.totalSats = amount;
                    },
                    Err(_) => {
                        error!("Error, Invalid conversion to satoshis");
                        return Err(CustomError::new(400, "Error converting".to_string()));
                    }
                };
            }

            let preorder: Preorder = paso1_api_business_pre_order(preventa.clone()).await?;
            
            if preorder.success == true {
                let presell_id = preorder.data.uid.clone();
                let split_listdets: Vec<PreorderSplit> = preorder.data.paymentSplit.clone();

                let ln = paso2_api_lightning_invoice(headers, presell_id, split_listdets, preventa.clone()).await?;

                if ln.data.business_id > 0 {
                    info!("business_id {:?}",ln.data.business_id);
                    Ok(ln)
                } else  {  
                    error!("Error, invoice was not saved in ApiLightning");
                    Err(CustomError::new(400, "Error, invoice was not saved in ApiLightning".to_string()))
                } 
            } else {
                error!("Error, invoice was not saved in ApiBusiness: {:?}", preorder.message);
                Err(CustomError::new(400, "Error, invoice was not saved in ApiBusiness".to_string()))
            }    
        }, 
        Err(_) => {
            error!("Error, Invalid conversion to satoshis");
            Err(CustomError::new(400, "Invalid conversion to satoshis".to_string()))
        }    
    }


}  


fn is_valid_row_details(det: &NewInvoiceDet, amount_min: BigDecimal) -> bool {
    let _cond1 = det.productId > 0;
    let _cond2 = det.productName.trim().len() > 0;
    let _cond3 = det.quantity > 0;
    let _cond4 = det.subTotal > BigDecimal::zero() &&
                 det.subTotal >= amount_min;
    let _cond5 = det.grandTotal > BigDecimal::zero();

    _cond1 & _cond2 & _cond3 & _cond4 & _cond5
} 

pub async fn paso1_api_business_pre_order(preventa: NewInvoice) -> Result<Preorder, CustomError> {
    match api_business_auth().await { 
        Ok(jwt) => {
            let socket: String;
            socket = CONFIG.api.api_server.to_string();
    
            // Construct the request
            let client = Client::builder().build()?; 
            let response = client
                    .post(socket.to_owned() + "/api/v1/order/create")
                    .header("Authorization", format!("Bearer {}", jwt))
                    .header(header::CONTENT_TYPE, "application/json") 
                    .json(&preventa)
                    .send()
                    .await?;
    
            // Check the response body
            let body = response.text().await?;
            info!("Response Body PreOrder: {:?}", body);
            
            // Deserialize JSON into struct
            let result: Result<Preorder, serde_json::Error> = serde_json::from_str(&body);
    
            match result {
                Ok(preOrder) => {
                    if preOrder.success {
                        info!("Consulted payment split: {:?}", preOrder);
    
                        // Construct the request
                        let client = Client::builder().build()?; 
                        let response = client
                                .get(socket.to_string() + "/api/v1/order/" + &preOrder.data.uid.to_string())
                                .header("Authorization", format!("Bearer {}", jwt))
                                .header(header::CONTENT_TYPE, "application/json") 
                                .send()
                                .await?;
    
                        // Check the response body
                        let body = response.text().await?;
                        info!("Response Body Split: {:?}", body);
                        
                        // Deserialize JSON into struct
                        let consult: Result<Preorder, serde_json::Error> = serde_json::from_str(&body);
    
                        match consult {
                            Ok(preOrder) => {
                                if preOrder.success {
                                    info!("Deserialized preOrder: {:?}", preOrder);
                                    Ok(preOrder)    
                                } else {
                                    error!("Error saving preorder...");
                                    Err(CustomError::new(400, "Error saving preorder".to_string()))
                                }
                            }
                            Err(e) => {
                                error!("Error deserialized: {:?}", e);
                                Err(CustomError::new(400, e.to_string()))
                            }
                        }
                    } else {
                        error!("Error saving preorder ...");
                        Err(CustomError::new(400, "Error saving preorder".to_string()))
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


pub async fn paso2_api_lightning_invoice(headers: &HeaderMap, presell_id: Uuid, split_listdets: Vec<PreorderSplit>, preventa: NewInvoice) -> Result<DataInvoice, CustomError> {
    let auth_encoded = headers.get("Authorization").expect("Error token").to_str().unwrap_or_default().to_string();

    let socket: String;
    socket = CONFIG.server.host.to_string() + ":8181";

    let description1 = "TorogozPay => BusinessId: ".to_string() + &preventa.businessId.to_string();
    let description2 = "/ OrderId: ".to_owned() + &preventa.orderId.to_string();
    let description3 =  "/ Currency: ".to_owned() + &preventa.currency.to_string();
    let description4 =  "/ Amount: ".to_owned() + &preventa.totalAmount.to_f64().unwrap().to_string();
    let description = description1 + &description2 + &description3 + &description4; 
    
    let invoice = InvoiceData {
        business_id: preventa.businessId,
        order_id: preventa.orderId,
        presell_id: presell_id,
        customer_name: preventa.customerName,
        customer_email: preventa.customerEmail,
        invoice_date: preventa.invoiceDate,
        description: description,
        currency: preventa.currency,
        sub_total: preventa.subTotal.clone(),
        taxes: preventa.taxes.clone(),
        shipping: preventa.shipping.clone(),
        total_amount: preventa.totalAmount.clone(),
        amount_sat: preventa.totalSats.clone() as i64,
        apply_split: preventa.applySplit,
        paymentSplit: split_listdets
    };    

    // Construct the request
    let client = Client::builder().build()?; 
    let response = client
            .post("http://".to_owned() + &socket.to_string() + "/api/lightning/v1/createInvoice")
            .header("Authorization", &auth_encoded)
            .header(header::CONTENT_TYPE, "application/json") 
            .json(&invoice)
            .send()
            .await?;

    // Check the response body
    let body = response.text().await?;
    info!("Response Body Invoice: {:?}", body);

    // Deserialize JSON into struct
    let result: Result<DataInvoice, serde_json::Error> = serde_json::from_str(&body);

    match result {
        Ok(invoice) => {
            info!("Deserialized Invoice: {:?}", invoice);
            return Ok(invoice);    
        }
        Err(e) => {
            error!("Error deserialized: {:?}", e);
            return Err(CustomError::new(400, e.to_string()));
        }
    }
}