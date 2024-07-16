-- Your SQL goes here
CREATE TABLE businesses (
 id serial primary key,
 app_name varchar(100) not null,
 app_logo varchar(100) not null,
 app_url varchar(100) not null,
 api_id varchar(32) not null,
 api_secret varchar(32) not null,
 workspace_id uuid not null,
 notify_customer boolean not null default false,
 notify_email boolean not null default false, 
 set_emails varchar(100),
 notify_webhook boolean not null default false, 
 url_webhook varchar(100),
 url_redirect varchar(100),
 link_url_pay varchar(100),
 link_timeout int not null default 0,
 link_amount boolean not null default false,
 link_count boolean not null default false,
 ask_name boolean not null default false, 
 ask_mobile boolean not null default false, 
 ask_email boolean not null default false, 
 ask_address boolean not null  default false,
 created_at timestamp with time zone not null default now(),
 updated_at timestamp with time zone null,
 enabled boolean not null default true,
 apply_split boolean not null default true,
 ln_address varchar(100) not null,
);

create unique index idx_cn_businesses on businesses (
 api_id
);

CREATE TABLE currencies (
 id serial primary key,
 currency varchar not null,
 yadio varchar not null,
 binance varchar not null
);


INSERT INTO currencies(currency, yadio, binance)
VALUES('USD','USD','BTCUSDT');


CREATE TABLE users (
 id serial primary key,
 email varchar not null,
 password varchar not null
);

CREATE TABLE configuration (
 id serial primary key,
 amount_min numeric(9,2) not null default 1
);