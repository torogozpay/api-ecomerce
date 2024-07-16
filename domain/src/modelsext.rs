// domain/src/modelsext.rs
 #![allow(non_snake_case)]

 use serde::{Deserialize, Serialize}; 
 use utoipa::ToSchema;
 use chrono::{Utc, DateTime};
 use bigdecimal::BigDecimal;
 use uuid::Uuid;

// Define a structure to represent the token claims
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
}

// Structure to represent request data
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RequestData {
  pub grant_type: Option<String>,
  pub client_id: Option<String>,
  pub client_secret: Option<String>,
  pub audience: Option<String>,  // New property for the audience
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct CreateToken {
  pub username: String,
  pub password: String
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct Token {
  pub jwt: String,
}

 #[derive(Debug, Serialize, Deserialize, Default, utoipa::IntoParams, ToSchema)]
 pub struct BusinessCreated {
   pub api_id: String,
   pub api_secret: String
 }

 #[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
 pub struct InvoiceData {
   pub business_id: i32,
   pub order_id: i32,
   pub presell_id: Uuid,
   pub customer_name: String,
   pub customer_email: String,
   pub invoice_date: DateTime<Utc>,   
   pub description: String,
   pub currency: String,
   pub sub_total: BigDecimal,
   pub taxes: BigDecimal,
   pub shipping: BigDecimal,
   pub total_amount: BigDecimal,
   pub amount_sat: i64,
   pub apply_split: bool,
   pub paymentSplit: Vec<PreorderSplit>   
 }

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct NewInvoice {
    pub businessId: i32,
    pub orderId: i32,
    pub customerName: String,
    pub customerEmail: String,
    pub currency: String,
    #[schema(value_type = String)]
    pub subTotal: BigDecimal,
    #[schema(value_type = String)]
    pub taxes: BigDecimal,
    #[schema(value_type = String)]
    pub shipping: BigDecimal,
    #[schema(value_type = String)]
    pub totalAmount: BigDecimal,
    pub totalSats: i64,
    #[schema(value_type = String, format = Date)]
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
   #[schema(value_type = String)]
   pub subTotal: BigDecimal,
   #[schema(value_type = String)]
   pub taxes: BigDecimal,
   #[schema(value_type = String)]
   pub grandTotal: BigDecimal,
   pub totalSats: i64
}


impl NewInvoiceDet {
    pub fn new(productId: i32, productName: String, quantity: i32, subTotal: BigDecimal, taxes: BigDecimal, grandTotal: BigDecimal, totalSats: i64) -> NewInvoiceDet{
        return NewInvoiceDet {
           productId: productId,
           productName: productName,
           quantity: quantity,
           subTotal: subTotal,
           taxes: taxes,
           grandTotal: grandTotal,
           totalSats: totalSats
        }

    }
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct EmpMessage {
    pub success: bool,
    pub message: String,
    pub data: PreorderEmp
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct Preorder {
    pub success: bool,
    pub message: String,
    pub data: PreorderDet
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct PreorderDet {
  pub uid: Uuid,    
  pub invoiceAddress: Option<String>,
  pub invoiceDate: DateTime<Utc>, 
  pub status: i32,  
  pub empresa: PreorderEmp,
  pub applySplit: bool,
  pub currency: String,
  pub totalAmount: BigDecimal,
  pub orderId: i32,
  pub amountSat: i64,
  pub createdAt: Option<DateTime<Utc>>, 
  pub updateAt: Option<DateTime<Utc>>, 
  pub distributed: bool,
  pub customerName: Option<String>,
  pub customerEmail: Option<String>,
  pub subTotal: BigDecimal,
  pub taxes: BigDecimal,
  pub shipping: BigDecimal,
  pub paymentSplit: Vec<PreorderSplit>
}  

#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct PreorderEmp {
  pub id: i32,
  pub nombre: Option<String>,
  pub descripcion: Option<String>,
  pub activo: i32,
  pub apiId: Option<String>,
  pub secretId: Option<String>,
  pub lnAddress: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct PreorderSplit {
  pub invoiceUid: Uuid,  
  pub tipoAsociado: String,
  pub ldAddress: String,
  pub amountSat: i32,
  pub status: i32,  
  pub invoiceAddress: Option<String>, 
  pub attempts: i32,    
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
 pub struct OrderFilters {
   pub uuid: String
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
pub struct DataInvoice {
    pub data: InvoiceResponse,
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct InvoiceResponse {
    pub business_id: i32,
    pub woocomerce_id: i32,    
    #[schema(value_type = String)]
    pub tpay_preorder_id: Uuid,    
    pub invoice_request: String,
    pub hash: Option<String>,
    pub paid: bool,
    pub result: String,
    pub code: i32,
    pub message: String
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct DataLookupInvoice {
    pub data: LookupInvoiceResponse,
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)] 
pub struct LookupInvoiceResponse {
  pub business_id: i32,
  pub woocomerce_id: i32,    
  #[schema(value_type = String)]
  pub tpay_preorder_id: Uuid,    
  pub hash: Option<String>,
  pub currency: String,
  #[schema(value_type = String)]
  pub totalAmount: BigDecimal,
  pub memo: String,
  pub r_preimage: String,
  pub r_hash: String,
  pub value: i64,
  pub value_msat: i64,
  pub settled: bool,
  pub settle_date: i64,
  pub creation_date: i64,
  pub payment_request: String,
  pub expiry: i64,
  pub cltv_expiry: u64,
  pub private: bool,
  pub add_index: u64,
  pub settle_index: u64,
  pub amt_paid: i64,
  pub amt_paid_sat: i64,
  pub amt_paid_msat: i64,
  pub state: i32,
  pub paid: bool,
  pub result: String,
  pub code: i32,
  pub message: String  
}


#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct InvoiceFilters { 
  pub hash: String 
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct CurrencyFilters {
  pub currency: String,
  #[schema(value_type = String)]
  pub total_amount: BigDecimal,   
}

 #[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Default)]
 pub struct Invoice {
    pub id: i32,
    pub business_id: i32,
    pub order_id: i32,
    #[schema(value_type = String)]
    pub presell_id: Uuid,
    pub bolt11: String,
    pub payment_hash: Option<String>,
    pub payment_secret: Option<String>,
    pub description: String,
    pub customer_name: String,
    pub customer_email: String,
    pub currency: String,
    #[schema(value_type = String)]
    pub sub_total: BigDecimal, 
    #[schema(value_type = String)]
    pub taxes: BigDecimal, 
    #[schema(value_type = String)]
    pub shipping: BigDecimal, 
    #[schema(value_type = String)]
    pub total_amount: BigDecimal, 
    pub amount_sat: i64, 
    pub status: i32,
    #[schema(value_type = String)]
    pub invoice_date: DateTime<Utc>,
    #[schema(value_type = String)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String)]
    pub updated_at: Option<DateTime<Utc>>,
    pub distributed: bool,
    pub apply_split: bool,
    pub result: String,
    pub code: i32,
    pub message: String  
  
 }