// domain/src/models.rs
 extern crate uuid;

 use crate::schema::*;
 use chrono::{Utc, DateTime};
 use diesel::prelude::*;
 use serde::{Deserialize, Serialize}; 
 use std::cmp::{Ord, Eq, PartialOrd, PartialEq};
 use uuid::Uuid;

 use serde_with::{serde_as, DisplayFromStr};

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
     pub apply_split: bool,
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
     pub apply_split: bool,     
 }

 
 #[serde_as]
 #[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Selectable, Ord, Eq, PartialOrd, PartialEq, ToSchema, Clone)]
 #[diesel(table_name = currencies)]
 pub struct Currencies {
    pub id: i32,
    pub currency: String,
    pub yadio: String,
    pub binance: String,
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