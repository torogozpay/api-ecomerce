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
    }
}

diesel::table! {
    businesses_nodes (id) {
        id -> Int4,
        business_id -> Int4,
        node_id -> Text,
        lnd -> Bool,
        host -> Text,
        port -> Int4,
        macaroon -> Text,
        cert -> Text,
        path -> Text,
        expiry -> Int4,
        cltv -> Int4,
        max_paths -> Int4,
        pathfinding_timeout -> Int4,
        max_fee -> Numeric,
        out -> Text,
    }
}

diesel::table! {
    invoices (id) {
        id -> Int4,
        business_id -> Int4,
        bolt11 -> Nullable<Varchar>,
        payment_hash -> Nullable<Varchar>,
        payment_secret -> Nullable<Varchar>,
        #[max_length = 250]
        description -> Varchar,
        label -> Nullable<Varchar>,
        amount -> Numeric,
        payment_address -> Nullable<Varchar>,
        #[max_length = 25]
        payment_status -> Nullable<Varchar>,
        invoice_date -> Timestamptz,
        #[max_length = 50]
        first_name -> Varchar,
        #[max_length = 50]
        last_name -> Varchar,
        #[max_length = 60]
        email -> Varchar,
        #[max_length = 25]
        phone_number -> Varchar,
        #[max_length = 100]
        address -> Varchar,
        #[max_length = 50]
        city -> Varchar,
        #[max_length = 20]
        id_country -> Varchar,
        #[max_length = 20]
        id_region -> Varchar,
        #[max_length = 20]
        postal_code -> Varchar,
        #[max_length = 100]
        url_redirect -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    invoices_det (id) {
        id -> Int4,
        invoice_id -> Int4,
        #[max_length = 30]
        product_code -> Varchar,
        quantity -> Numeric,
        amount -> Numeric,
    }
}

diesel::joinable!(businesses_nodes -> businesses (business_id));
diesel::joinable!(invoices -> businesses (business_id));
diesel::joinable!(invoices_det -> invoices (invoice_id));

diesel::allow_tables_to_appear_in_same_query!(
    businesses,
    businesses_nodes,
    invoices,
    invoices_det,
);
