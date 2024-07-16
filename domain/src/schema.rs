// @generated automatically by Diesel CLI.

diesel::table! {
    businesses (id) {
        id -> Int4,
        #[max_length = 100]
        app_name -> Varchar,
        #[max_length = 100]
        app_logo -> Varchar,
        #[max_length = 100]
        app_url -> Varchar,
        #[max_length = 32]
        api_id -> Varchar,
        #[max_length = 32]
        api_secret -> Varchar,
        workspace_id -> Uuid,
        notify_customer -> Bool,
        notify_email -> Bool,
        #[max_length = 100]
        set_emails -> Nullable<Varchar>,
        notify_webhook -> Bool,
        #[max_length = 100]
        url_webhook -> Nullable<Varchar>,
        #[max_length = 100]
        url_redirect -> Nullable<Varchar>,
        #[max_length = 100]
        link_url_pay -> Nullable<Varchar>,
        link_timeout -> Int4,
        link_amount -> Bool,
        link_count -> Bool,
        ask_name -> Bool,
        ask_mobile -> Bool,
        ask_email -> Bool,
        ask_address -> Bool,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
        enabled -> Bool,
        apply_split -> Bool,
        #[max_length = 100]
        ln_address -> Varchar,
    }
}

diesel::table! {    
    currencies (id) {
        id -> Int4,
        currency -> Varchar,
        yadio -> Varchar,
        binance -> Varchar,
    }
}

diesel::table! {    
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::table! {    
    configuration (id) {
        id -> Int4,
        amount_min -> Numeric,
    }
}