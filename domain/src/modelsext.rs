// domain/src/modelsext.rs
 #![allow(non_snake_case)]

 use serde::{Deserialize, Serialize}; 
 use utoipa::ToSchema;
 use chrono::{Utc, DateTime};
 use bigdecimal::BigDecimal;
 use uuid::Uuid;

/// Define a structure to represent the token claims
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
}

/// Define a structure to represent data to generate token
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RequestData {
  /// Represents the grant type
  pub grant_type: Option<String>,
  /// Represents the identifier of the API
  pub client_id: Option<String>,
  /// Represents the secret of the API
  pub client_secret: Option<String>,
  /// Represents the audience
  pub audience: Option<String>,  
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

 /// Define a structure to show data of the API
 #[derive(Debug, Serialize, Deserialize, Default, utoipa::IntoParams, ToSchema)]
 pub struct BusinessCreated {
  /// Represents the identifier of the API 
  pub api_id: String,
  /// Represents the secret of the API
  pub api_secret: String
 }

 /// Define a structure to represent data of the invoice
 #[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
 pub struct InvoiceData {
  /// Represents the identifier of the business
  pub business_id: i32,
  /// Represents the identifier of the order
  pub order_id: i32,
  /// Represents the identifier of the pre sale
  pub presell_id: Uuid,
  /// Represents the name of the customer
  pub customer_name: String,
  /// Represents the email of the email
  pub customer_email: String,
  /// Represents the date of the invoice
  pub invoice_date: DateTime<Utc>,   
  /// Represents the description of the order
  pub description: String,
  /// Represents the currency of the transaction
  pub currency: String,
  /// Represents the subtotal of the order 
  pub sub_total: BigDecimal,
  /// Represents the taxes of the order
  pub taxes: BigDecimal,
  /// Represents the shipping of the order
  pub shipping: BigDecimal,
  /// Represents the total amount of the order
  pub total_amount: BigDecimal,
  /// Represents the total sats of the order 
  pub amount_sat: i64,
  /// Represents whether split payment applies
  pub apply_split: bool,
  /// Represents the details of products of the order
  pub paymentSplit: Vec<PreorderSplit>   
 }

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct NewInvoice {
    /// Represents the identifier of the business
    pub businessId: i32,
    /// Represents the identifier of the order
    pub orderId: i32,
    /// Represents the name of the customer
    pub customerName: String,
    /// Represents the email of the email
    pub customerEmail: String,
    /// Represents the currency of the transaction
    pub currency: String,
    /// Represents the subtotal of the order 
    #[schema(value_type = String)]
    pub subTotal: BigDecimal,
    /// Represents the taxes of the order
    #[schema(value_type = String)]
    pub taxes: BigDecimal,
    /// Represents the shipping of the order
    #[schema(value_type = String)]
    pub shipping: BigDecimal,
    /// Represents the total amount of the order
    #[schema(value_type = String)]
    pub totalAmount: BigDecimal,
    /// Represents the total sats of the order 
    pub totalSats: i64,
    /// Represents the date of the order
    #[schema(value_type = String, format = Date)]
    pub invoiceDate: DateTime<Utc>,
    /// Represents the datetime of the transaction
    pub invoiceStamp: String,
    /// Represents whether split payment applies
    pub applySplit: bool,
    /// Represents the details of products of the order
    pub details: Vec<NewInvoiceDet>
 }
 
/// Define a structure to represent data of the products of the invoice
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)] 
pub struct NewInvoiceDet {
  /// Represents the identifier of the product
  pub productId: i32,
  /// Represents the name of the product
  pub productName: String,
  /// Represents the quantity of the product
  pub quantity: i32,
  /// Represents the subtotal of the product
  #[schema(value_type = String)]
  pub subTotal: BigDecimal,
  /// Represents the taxes of the product
  #[schema(value_type = String)]
  pub taxes: BigDecimal,
  /// Represents the grand total of the product
  #[schema(value_type = String)]
  pub grandTotal: BigDecimal,
  /// Represents the total sats of the product
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

/// Define a structure to represent the filter of the order
#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct OrderFilters {
  /// Represents the identifier of the order
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

/// Define a structure to show data of the invoice
#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct InvoiceResponse {
  /// Represents the identifier of the business
  pub business_id: i32,
  /// Represents the identifier of the order
  pub woocomerce_id: i32,    
  /// Represents the identifier of the order (Uuid)
  #[schema(value_type = String)]
  pub tpay_preorder_id: Uuid,    
  /// Represents the request of the invoice
  pub invoice_request: String,
  /// Represents the hash of the invoice
  pub hash: Option<String>,
  /// Indicates whether the invoice was paid
  pub paid: bool,
  /// Represents the result of the transaction
  pub result: String,
  /// Represents the code of the result of the transaction
  pub code: i32,
  /// Represents the message of the transaction
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

/// Define a structure to filter the invoice by hash
#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct InvoiceFilters { 
  /// Represents the hash of the invoice
  pub hash: String 
}

/// Define a structure to save currency and total amount of the transaction
#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
pub struct CurrencyFilters {
  /// Represents the currency
  pub currency: String,
  /// Represents the total amount
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