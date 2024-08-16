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
 
 /// Define a structure to represent data of the businesses
 #[serde_as]
 #[derive(Debug, Serialize, Deserialize, Selectable, Insertable, AsChangeset, Clone, ToSchema)]
 #[diesel(table_name = businesses)]
 pub struct NewBusiness {
     /// Represents the name of the application (Business)
     pub app_name: String,
     /// Represents the route of the application logo
     pub app_logo: String,
     /// Represents the URL of the application
     pub app_url: String,
     /// Represents the identifier of the API 
     pub api_id: String,
     /// Represents the secret of the API
     pub api_secret: String,
     /// Represents the identifier of the workspace 
     #[schema(value_type = String)]
     #[serde(with = "my_uuid")]
     pub workspace_id: Uuid,
     /// Indicates whether the customer must be notified
     pub notify_customer: bool,
     /// Indicates whether the customer must be notified by email
     pub notify_email: bool,
     /// Represents the emails to notify
     pub set_emails: Option<String>,
     /// Represents the webhook to notify 
     pub notify_webhook: bool,
     /// Represents the URL of the webhook 
     pub url_webhook: Option<String>,
     /// Represents the URL to redirect
     pub url_redirect: Option<String>,
     /// Represents the URL of the payment link 
     pub link_url_pay: Option<String>,
     /// Represents the timeout of the link
     pub link_timeout: i32,
     /// Represents the amount of the link
     pub link_amount: bool,
     /// Represents the count of the link
     pub link_count: bool,
     /// Indicates whether ask name
     pub ask_name: bool,
     /// Indicates whether ask mobile
     pub ask_mobile: bool,
     /// Indicates whether ask email
     pub ask_email: bool,
     /// Indicates whether ask address
     pub ask_address: bool,
     /// Represents the date of creation
     #[schema(value_type = String, format = Date)]
     #[serde_as(as = "DisplayFromStr")]
     pub created_at: DateTime<Utc>,
     /// Represents the update date
     #[schema(value_type = String, format = Date)]
     pub updated_at: Option<DateTime<Utc>>,     
     /// Represents whether is an enabled business
     pub enabled: bool,
     /// Represents whether split payment applies
     pub apply_split: bool,
     /// Represents the address LNURL
     pub ln_address: String,
 }

/// Define a structure to represent data of the businesses
#[serde_as]
 #[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, Ord, Eq, PartialOrd, PartialEq, Clone, ToSchema)] 
 #[diesel(table_name = businesses)]
 pub struct Business {
     /// Represents the identifier of the business
     pub id: i32,
     /// Represents the name of the application (Business)
     pub app_name: String,
     /// Represents the route of the application logo
     pub app_logo: String,
     /// Represents the URL of the application
     pub app_url: String,
     /// Represents the identifier of the API 
     pub api_id: String,
     /// Represents the secret of the API
     pub api_secret: String,
     /// Represents the identifier of the workspace 
     #[schema(value_type = String)]
     #[serde(with = "my_uuid")]
     pub workspace_id: Uuid,
     /// Indicates whether the customer must be notified
     pub notify_customer: bool,
     /// Indicates whether the customer must be notified by email
     pub notify_email: bool,
     /// Represents the emails to notify
     pub set_emails: Option<String>,
     /// Represents the webhook to notify 
     pub notify_webhook: bool,
     /// Represents the URL of the webhook 
     pub url_webhook: Option<String>,
     /// Represents the URL to redirect
     pub url_redirect: Option<String>,
     /// Represents the URL of the payment link 
     pub link_url_pay: Option<String>,
     /// Represents the timeout of the link
     pub link_timeout: i32,
     /// Represents the amount of the link
     pub link_amount: bool,
     /// Represents the count of the link
     pub link_count: bool,
     /// Indicates whether ask name
     pub ask_name: bool,
     /// Indicates whether ask mobile
     pub ask_mobile: bool,
     /// Indicates whether ask email
     pub ask_email: bool,
     /// Indicates whether ask address
     pub ask_address: bool,
     /// Represents the date of creation
     #[schema(value_type = String, format = Date)]
     #[serde_as(as = "DisplayFromStr")]
     pub created_at: DateTime<Utc>,
     /// Represents the update date
     #[schema(value_type = String, format = Date)]
     pub updated_at: Option<DateTime<Utc>>,     
     /// Represents whether is an enabled business
     pub enabled: bool,
     /// Represents whether split payment applies
     pub apply_split: bool,
     /// Represents the address LNURL
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