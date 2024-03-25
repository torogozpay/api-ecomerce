// domain/src/modelsext.rs
 #![allow(non_snake_case)]

 use serde::{Deserialize, Serialize}; 
 use utoipa::ToSchema;
 use chrono::{Utc, DateTime};
 use bigdecimal::BigDecimal;

// Definir una estructura para representar los claims del token
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
}

 #[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
 pub struct BusinessCreated {
   pub api_id: String,
   pub api_secret: String
 }

 #[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
 pub struct InvoiceData {
   pub business_id: i32,
   pub order_id: i32,
   pub presell_id: i32,
   pub split_id: i32,
   pub name: String,
   pub email: String,
   pub invoice_date: DateTime<Utc>,   
   pub description: String,
   pub currency: String,
   pub total_amount: BigDecimal,
   pub amount_msat: i64,
   pub apply_split: bool,
 }

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct NewInvoice {
    pub businessId: i32,
    pub orderId: i32,
    pub customerName: String,
    pub customerEmail: String,
    pub currency: String,
    pub subTotal: BigDecimal,
    pub taxes: BigDecimal,
    pub shipping: BigDecimal,
    pub totalAmount: BigDecimal,
    pub invoiceDate: DateTime<Utc>,
    pub invoiceStamp: String,
    pub applySplit: bool,
    pub details: Vec<NewInvoiceDet>
 }
 
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)] 
pub struct NewInvoiceDet {
   pub productId: i32,
   pub productName: String,
   pub quantity: i32,
   pub subTotal: BigDecimal,
   pub taxes: BigDecimal,
   pub grandTotal: BigDecimal
}


impl NewInvoiceDet {
    pub fn new(productId: i32, productName: String, quantity: i32, subTotal: BigDecimal, taxes: BigDecimal, grandTotal: BigDecimal) -> NewInvoiceDet{
        return NewInvoiceDet {
           productId: productId,
           productName: productName,
           quantity: quantity,
           subTotal: subTotal,
           taxes: taxes,
           grandTotal: grandTotal
        }

    }
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct Preorder {
    pub presell_id: i32,    
    pub split_id: i32
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct InvoiceAddress {
  pub address: String,
  pub amount: i64
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct InfoResponse {
    pub identity_pubkey: String,
    pub alias: Option<String>,
    pub block_height: String,
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct InvoiceResponse {
    pub business_id: i32,
    pub order_id: i32,    
    pub payment_request: Option<String>,
    pub preimage: Option<String>,
    pub hash: Option<String>,
    pub paid: bool
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct InvoiceFilters {
  pub hash: String 
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct CurrencyFilters {
  pub currency: String,
  pub total_amount: BigDecimal,   
}