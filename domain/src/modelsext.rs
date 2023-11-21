use serde::{Deserialize, Serialize}; 
use bigdecimal::BigDecimal;


#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InvoiceExt {
  pub method: String,
  pub result: InvoiceResult
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InvoiceResult {
  pub bolt11: String,
  pub payment_hash: String,
  pub payment_secret: String,
  pub expires_at: BigDecimal,
  pub created_index: BigDecimal,
  pub warning_capacity: Option<String>,
  pub warning_offline: Option<String>,
  pub warning_deadends: Option<String>,
  pub warning_private_unused: Option<String>,
  pub warning_mpp: Option<String>   
 }



#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ListInvoicesExt {
  pub method: String,
  pub result: Vec<Invoices>
}


#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Invoices {
  pub label: String,
  pub description: String,
  pub payment_hash: String,
  pub status: String,
  pub expires_at: BigDecimal,
  pub amount_msat: String,
  pub bolt11: String,
  pub created_index: BigDecimal,
  pub updated_index: BigDecimal
}


#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InvoiceCreated {
  pub description: String,
  pub amount: BigDecimal
}