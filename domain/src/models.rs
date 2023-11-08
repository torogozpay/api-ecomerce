// domain/src/models.rs
 extern crate uuid;

 use crate::schema::*;
 use chrono::{Utc, DateTime};
 use diesel::prelude::*;
 use serde::{Deserialize, Serialize}; 
 use std::cmp::{Ord, Eq, PartialOrd, PartialEq};
 use uuid::Uuid;

 use serde_with::{serde_as, DisplayFromStr};
 use bigdecimal::BigDecimal;
 use utoipa::ToSchema;

 #[serde_as]
 #[derive(Debug, Serialize, Deserialize, Selectable, Insertable, AsChangeset, ToSchema)]
 #[diesel(table_name = businesses)]
 pub struct NewBusiness {
     pub app_name: String,
     pub app_logo: String,
     pub app_url: String,
     pub api_id: String,
     pub api_secret: String,
     #[serde(with = "my_uuid")]
     pub workspace_id: Uuid,
     pub notify_customer: bool,
     pub notify_email: bool,
     pub set_emails: Option<String>,
     pub notify_webhook: bool,
     pub url_webhook: Option<String>,
     pub url_redirect: Option<String>,
     pub link_url_pay: Option<String>,
     pub link_timeout: i32,
     pub link_amount: bool,
     pub link_count: bool,
     pub ask_name: bool,
     pub ask_mobile: bool,
     pub ask_email: bool,
     pub ask_address: bool,
     #[serde_as(as = "DisplayFromStr")]
     pub created_at: DateTime<Utc>,
     pub updated_at: Option<DateTime<Utc>>,     
     pub enabled: bool,
 }

 #[serde_as]
 #[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, Ord, Eq, PartialOrd, PartialEq, ToSchema)] 
 #[diesel(table_name = businesses)]
 pub struct Business {
     pub id: i32,
     pub app_name: String,
     pub app_logo: String,
     pub app_url: String,
     pub api_id: String,
     pub api_secret: String,
     #[serde(with = "my_uuid")]
     pub workspace_id: Uuid,
     pub notify_customer: bool,
     pub notify_email: bool,
     pub set_emails: Option<String>,
     pub notify_webhook: bool,
     pub url_webhook: Option<String>,
     pub url_redirect: Option<String>,
     pub link_url_pay: Option<String>,
     pub link_timeout: i32,
     pub link_amount: bool,
     pub link_count: bool,
     pub ask_name: bool,
     pub ask_mobile: bool,
     pub ask_email: bool,
     pub ask_address: bool,
     #[serde_as(as = "DisplayFromStr")]
     pub created_at: DateTime<Utc>,
     pub updated_at: Option<DateTime<Utc>>,
     pub enabled: bool,
 }

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Selectable, Insertable, AsChangeset, Associations, ToSchema)]
#[diesel(belongs_to(Business, foreign_key = business_id))]
#[diesel(table_name = invoices)]
pub struct NewInvoice {
   pub business_id: i32,
   pub bolt11: Option<String>,
   pub payment_hash: Option<String>,
   pub payment_secret: Option<String>,
   #[serde_as(as = "DisplayFromStr")]
   pub expires_at: BigDecimal,
   #[serde_as(as = "DisplayFromStr")]
   pub created_index: BigDecimal,
   pub warning_capacity: Option<String>,
   pub warning_offline: Option<String>,
   pub warning_deadends: Option<String>,
   pub warning_private_unused: Option<String>,
   pub warning_mpp: Option<String>,
   pub description: String,
   #[serde_as(as = "DisplayFromStr")]
   pub amount: BigDecimal, 
   pub payment_address: Option<String>,
   pub payment_status: Option<String>,
   #[serde_as(as = "DisplayFromStr")]
   pub invoice_date: DateTime<Utc>,
   pub first_name: String,
   pub last_name: String,
   pub email: String,
   pub phone_number: String,
   pub address: String,
   pub city: String,
   pub id_country: String,
   pub id_region: String,
   pub postal_code: String,
   pub url_redirect: String,
   #[serde_as(as = "DisplayFromStr")]
   pub created_at: DateTime<Utc>,
   pub updated_at: Option<DateTime<Utc>>,      
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Selectable, Insertable, AsChangeset, Ord, Eq, PartialOrd, PartialEq, Associations, ToSchema)]
#[diesel(belongs_to(Business, foreign_key = business_id))]
#[diesel(table_name = invoices)]
pub struct Invoice {
   pub id: i32,
   pub business_id: i32,
   pub bolt11: Option<String>,
   pub payment_hash: Option<String>,
   pub payment_secret: Option<String>,
   #[serde_as(as = "DisplayFromStr")]
   pub expires_at: BigDecimal,
   #[serde_as(as = "DisplayFromStr")]
   pub created_index: BigDecimal,
   pub warning_capacity: Option<String>,
   pub warning_offline: Option<String>,
   pub warning_deadends: Option<String>,
   pub warning_private_unused: Option<String>,
   pub warning_mpp: Option<String>,
   pub description: String,
   #[serde_as(as = "DisplayFromStr")]
   pub amount: BigDecimal, 
   pub payment_address: Option<String>,
   pub payment_status: Option<String>,
   #[serde_as(as = "DisplayFromStr")]
   pub invoice_date: DateTime<Utc>,
   pub first_name: String,
   pub last_name: String,
   pub email: String,
   pub phone_number: String,
   pub address: String,
   pub city: String,
   pub id_country: String,
   pub id_region: String,
   pub postal_code: String,
   pub url_redirect: String,
   #[serde_as(as = "DisplayFromStr")]
   pub created_at: DateTime<Utc>,
   pub updated_at: Option<DateTime<Utc>>,
}
 
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, Associations, ToSchema)]
#[diesel(belongs_to(Invoice, foreign_key = invoice_id))]
#[diesel(table_name = invoices_det)]
pub struct NewInvoiceDet {
   pub invoice_id: i32,
   pub product_code: String,
   #[serde_as(as = "DisplayFromStr")]
   pub quantity: BigDecimal,
   #[serde_as(as = "DisplayFromStr")]
   pub amount: BigDecimal
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Selectable, Insertable, AsChangeset, Ord, Eq, PartialOrd, PartialEq, Associations, ToSchema)]
#[diesel(belongs_to(Invoice, foreign_key = invoice_id))]
#[diesel(table_name = invoices_det)]
pub struct InvoiceDet {
   pub id: i32,
   pub invoice_id: i32,
   pub product_code: String,
   #[serde_as(as = "DisplayFromStr")]
   pub quantity: BigDecimal,
   #[serde_as(as = "DisplayFromStr")]
   pub amount: BigDecimal
}
 
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, Associations, ToSchema)]
#[diesel(belongs_to(Business, foreign_key = business_id))]
#[diesel(table_name = businesses_nodes)]
pub struct NewBusinessNode {
   pub business_id: i32,
   pub node_id: i32,
   pub path: String,
   pub host: String,
   pub port: i32,
   pub expiry: i32,
   pub cltv: i32,
   pub max_paths: i32,
   pub pathfinding_timeout: i32,
   #[serde_as(as = "DisplayFromStr")]
   pub max_fee: BigDecimal,
   pub out: String  
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Selectable, Insertable, AsChangeset, Ord, Eq, PartialOrd, PartialEq, Associations, ToSchema)]
#[diesel(belongs_to(Business, foreign_key = business_id))]
#[diesel(table_name = businesses_nodes)]
#[diesel(primary_key(id))]
pub struct BusinessNode {
   pub id: i32,
   pub business_id: i32,
   pub node_id: i32,
   pub path: String,
   pub host: String,
   pub port: i32,
   pub expiry: i32,
   pub cltv: i32,
   pub max_paths: i32,
   pub pathfinding_timeout: i32,
   #[serde_as(as = "DisplayFromStr")]
   pub max_fee: BigDecimal,
   pub out: String  
} 

/*
#[derive(Serialize, Default)]
pub struct InvoiceResponse {
	bolt11: String,
	payment_hash: String,
	payment_secret: String,
	expires_at: u64,
	created_index: u64,
	warning_capacity: String,
	warning_offline: String,
	warning_deadends: String,
	warning_private_unused: String,
	warning_mpp: String,
}

#[derive(Serialize, Default)]
pub struct ListInvoices {
    invoices: Invoices
}

#[derive(Serialize, Default)]
pub struct Invoices {
    label: String,
    description: String,
	payment_hash: String,
	expires_at: u64,
	amount_msat: u64,
	bolt11: String,
}
 */

#[derive(Queryable, Serialize, Deserialize, ToSchema)] 
pub struct MyInvoice {
    pub api_secret: String,
    pub master: Invoice,
    pub details: Vec<InvoiceDet>
}

impl MyInvoice {
    pub fn new(api_secret: String, master: Invoice, details: Vec<InvoiceDet>) -> MyInvoice {
        MyInvoice {
            api_secret: api_secret,
            master: master,
            details: details
        }
    }
}

#[derive(Queryable, Serialize, Deserialize, ToSchema)] 
pub struct MyNewInvoice {
    pub api_secret: String,
    pub master: NewInvoice,
    pub details: Vec<NewInvoiceDet>
}

impl MyNewInvoice {
    pub fn new(api_secret: String, master: NewInvoice, details: Vec<NewInvoiceDet>) -> MyNewInvoice {
        MyNewInvoice {
            api_secret: api_secret,
            master: master,
            details: details
        }
    }
}

mod my_uuid {
    use uuid::Uuid;
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
    use std::str::FromStr;

    pub fn serialize<S>(val: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        val.to_string().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: &str = Deserialize::deserialize(deserializer)?;
        Uuid::from_str(val).map_err(D::Error::custom)
    }
}

impl NewInvoiceDet {
    pub fn new(invoice_id: i32,product_code: String,quantity: BigDecimal, amount: BigDecimal) -> NewInvoiceDet{
        return NewInvoiceDet {
           invoice_id: invoice_id,
           product_code: product_code,
           quantity: quantity,
           amount: amount
        }

    }
}

impl NewBusinessNode {
    pub fn new(business_id: i32, node_id: i32, path: String, host: String, port: i32, 
               expiry: i32, cltv: i32, max_paths: i32, pathfinding_timeout: i32,
               max_fee: BigDecimal, out: String) -> NewBusinessNode{
        return NewBusinessNode {
            business_id: business_id,
            node_id: node_id,
            path: path,
            host: host,
            port: port,
            expiry: expiry,
            cltv: cltv,
            max_paths: max_paths,
            pathfinding_timeout: pathfinding_timeout,
            max_fee: max_fee,
            out: out  
        }
    }
}