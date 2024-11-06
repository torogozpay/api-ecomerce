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
  /// Represents the API identifier
  pub client_id: Option<String>,
  /// Represents the API secret
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

 /// Define a data structure to show API‘s information 
 #[derive(Debug, Serialize, Deserialize, Default, utoipa::IntoParams, ToSchema)]
 pub struct BusinessCreated {
  /// Represents the API identifier 
  pub api_id: String,
  /// Represents the API secret
  pub api_secret: String
 }

 /// Define a structure to represent data of the invoice
 #[derive(Debug, Serialize, Deserialize, Default, ToSchema, Clone)]
 pub struct InvoiceData {
  /// Represents the business identifier
  pub business_id: i32,
  /// Represents the order‘s identifier
  pub order_id: i32,
  /// Represents the pre sale identifier
  pub presell_id: Uuid,
  /// Represents the customer‘s name
  pub customer_name: String,
  /// Represents the email address
  pub customer_email: String,
  /// Represents the invoice‘s date
  pub invoice_date: DateTime<Utc>,   
  /// Represents the order‘s description 
  pub description: String,
  /// Represents the transaction currency 
  pub currency: String,
  /// Represents the order‘s subtotal 
  pub sub_total: BigDecimal,
  /// Represents the order‘s taxes
  pub taxes: BigDecimal,
  /// Represents the order‘s shipping amount
  pub shipping: BigDecimal,
  /// Represents the order‘s total amount
  pub total_amount: BigDecimal,
  /// Represents the order‘s total sats  
  pub amount_sat: i64,
  /// Represents split payment if applicable
  pub apply_split: bool,
  /// Represents the order detail
  pub paymentSplit: Vec<PreorderSplit>   
 }

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct NewInvoice {
    /// Represents the business identifier
    pub businessId: i32,
    /// Represents the order‘s identifier
    pub orderId: i32,
    /// Represents the customer‘s name 
    pub customerName: String,
    /// Represents the email address
    pub customerEmail: String,
    /// Represents the transaction currency
    pub currency: String,
    /// Represents the order‘s subtotal  
    #[schema(value_type = String)]
    pub subTotal: BigDecimal,
    /// Represents the order‘s taxes
    #[schema(value_type = String)]
    pub taxes: BigDecimal,
    /// Represents the order‘s shipping amount
    #[schema(value_type = String)]
    pub shipping: BigDecimal,
    /// Represents the order‘s total amount 
    #[schema(value_type = String)]
    pub totalAmount: BigDecimal,
    /// Represents the order‘s total sats  
    pub totalSats: i64,
    /// Represents the order‘s date 
    #[schema(value_type = String, format = Date)]
    pub invoiceDate: DateTime<Utc>,
    /// Represents the transaction TimeStamp
    pub invoiceStamp: String,
    /// Represents split payment if applicable
    pub applySplit: bool,
    /// Represents the order detail
    pub details: Vec<NewInvoiceDet>
 }
 
/// Define a data structure to represent product‘s information included in the invoice
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)] 
pub struct NewInvoiceDet {
  /// Represents the product identifier 
  pub productId: i32,
  /// Represents the product‘s name 
  pub productName: String,
  /// Represents the product‘s quantity 
  pub quantity: i32,
  /// Represents the product‘s subtotal 
  #[schema(value_type = String)]
  pub subTotal: BigDecimal,
  /// Represents the product‘s taxes 
  #[schema(value_type = String)]
  pub taxes: BigDecimal,
  /// Represents the product‘s total amount
  #[schema(value_type = String)]
  pub grandTotal: BigDecimal,
  /// Represents the product‘s total sats
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
  /// Represents the order‘s identifier
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
  /// Represents Invoice hash
  pub hash: String 
}

/// Define data structure to save currency and total amount
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