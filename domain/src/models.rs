// domain/src/models.rs
 extern crate uuid;

 use crate::schema::*;
 use chrono::{Utc, DateTime};
 use diesel::prelude::*;
 use serde::{Deserialize, Serialize}; 
 use std::cmp::{Ord, Eq, PartialOrd, PartialEq};
 use uuid::Uuid;
 use crate::my_uuid;
 
 use serde_with::{serde_as, DisplayFromStr};
 use bigdecimal::BigDecimal;
 use utoipa::ToSchema;
 
 /// Define a structure to represent businesses data
 #[serde_as]
 #[derive(Debug, Serialize, Deserialize, Selectable, Insertable, AsChangeset, Clone, ToSchema)]
 #[diesel(table_name = businesses)]
 pub struct NewBusiness {
     /// Represents Business name
     pub app_name: String,
     /// Represents the location of the application‘s logo
     pub app_logo: String,
     /// Represents application‘s URL
     pub app_url: String,
     /// Represents the API identifier 
     pub api_id: String,
     /// Represents the API secret
     pub api_secret: String,
     /// Represents workspace identifier 
     #[schema(value_type = String)]
     #[serde(with = "my_uuid")]
     pub workspace_id: Uuid,
     /// Indicates Customer notification
     pub notify_customer: bool,
     /// Indicates Customer notification email
     pub notify_email: bool,
     /// Represents email addresses to notify
     pub set_emails: Option<String>,
     /// Represents notification webhook 
     pub notify_webhook: bool,
     /// Represents Webhook URL 
     pub url_webhook: Option<String>,
     /// Represents redirect URL
     pub url_redirect: Option<String>,
     /// Represents the payment‘s URL 
     pub link_url_pay: Option<String>,
     /// Represents the link timeout
     pub link_timeout: i32,
     /// Represents link‘s amount
     pub link_amount: bool,
     /// Represents link‘s count
     pub link_count: bool,
     /// Indicates whether name is required
     pub ask_name: bool,
     /// Indicates whether mobile phone number is required
     pub ask_mobile: bool,
     /// Indicates whether email is required
     pub ask_email: bool,
     /// Indicates whether address is required
     pub ask_address: bool,
     /// Issued date
     #[schema(value_type = String, format = Date)]
     #[serde_as(as = "DisplayFromStr")]
     pub created_at: DateTime<Utc>,
     /// Represents date of update
     #[schema(value_type = String, format = Date)]
     pub updated_at: Option<DateTime<Utc>>,     
     /// Represents an enabled business
     pub enabled: bool,
     /// Represents split payment if applicable
     pub apply_split: bool,
     /// Represents the LNURL address
     pub ln_address: String,
 }

/// Define a structure to represent businesses data
#[serde_as]
 #[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, Ord, Eq, PartialOrd, PartialEq, Clone, ToSchema)] 
 #[diesel(table_name = businesses)]
 pub struct Business {
     /// Represents a business identifier
     pub id: i32,
     /// Represents Business name
     pub app_name: String,
     /// Represents the location of the application‘s logo
     pub app_logo: String,
     /// Represents application‘s URL
     pub app_url: String,
     /// Represents the API identifier 
     pub api_id: String,
     /// Represents the API secret
     pub api_secret: String,
     /// Represents workspace identifier 
     #[schema(value_type = String)]
     #[serde(with = "my_uuid")]
     pub workspace_id: Uuid,
     /// Indicates Customer notification
     pub notify_customer: bool,
     /// Indicates Customer notification email
     pub notify_email: bool,
     /// Represents email addresses to notify
     pub set_emails: Option<String>,
     /// Represents notification webhook 
     pub notify_webhook: bool,
     /// Represents Webhook URL 
     pub url_webhook: Option<String>,
     /// Represents redirect URL
     pub url_redirect: Option<String>,
     /// Represents the payment‘s URL 
     pub link_url_pay: Option<String>,
     /// Represents the link timeout
     pub link_timeout: i32,
     /// Represents link‘s amount
     pub link_amount: bool,
     /// Represents link‘s count
     pub link_count: bool,
     /// Indicates whether name is required
     pub ask_name: bool,
     /// Indicates whether mobile phone number is required
     pub ask_mobile: bool,
     /// Indicates whether email is required
     pub ask_email: bool,
     /// Indicates whether address is required
     pub ask_address: bool,
     /// Issued date
     #[schema(value_type = String, format = Date)]
     #[serde_as(as = "DisplayFromStr")]
     pub created_at: DateTime<Utc>,
     /// Represents date of update
     #[schema(value_type = String, format = Date)]
     pub updated_at: Option<DateTime<Utc>>,     
     /// Represents an enabled business
     pub enabled: bool,
     /// Represents split payment if applicable
     pub apply_split: bool,
     /// Represents the LNURL address
     pub ln_address: String,
 }

 
 //#[serde_as]
 #[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Selectable, Ord, Eq, PartialOrd, PartialEq, Clone, ToSchema)]
 #[diesel(table_name = currencies)]
 pub struct Currencies {
    pub id: i32,
    pub currency: String,
    pub yadio: String,
    pub binance: String,
}

/// Define a structure to represent data of the user
//#[serde_as]
#[derive(Debug, Serialize, Deserialize, Selectable, Insertable, AsChangeset, Clone, ToSchema)]
#[diesel(table_name = users)]
pub struct NewUser {
    /// Represents the authorized email
    pub email: String,
    /// Represents the authorized password
    pub password: Option<String>
}


//#[serde_as]
#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, Ord, Eq, PartialOrd, PartialEq, Clone, ToSchema)] 
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String
}


#[serde_as]
#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, Ord, Eq, PartialOrd, PartialEq, Clone, ToSchema)] 
#[diesel(table_name = configuration)]
pub struct Configuration {
    pub id: i32,
    #[serde_as(as = "DisplayFromStr")]
    pub amount_min: BigDecimal,     
}