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
 enabled boolean not null default true
);


CREATE TABLE invoices (
 id serial primary key,
 business_id integer not null,
 bolt11 varchar null,
 payment_hash varchar null,
 payment_secret varchar null,
 expires_at numeric(18,0) not null default 0,
 created_index numeric(18,0) not null default 0,
 warning_capacity varchar null,
 warning_offline varchar null,
 warning_deadends varchar null,
 warning_private_unused varchar null,
 warning_mpp varchar null,
 description varchar(250) not null,
 amount numeric(18,2) not null default 0,
 payment_address varchar null,
 payment_status varchar(25) null,
 invoice_date timestamp with time zone not null,
 first_name varchar(50) not null,
 last_name varchar(50) not null,
 email varchar(60) not null,
 phone_number varchar(25) not null,
 address varchar(100) not null,
 city varchar(50) not null,
 id_country varchar(20) not null,
 id_region varchar(20) not null,
 postal_code varchar(20) not null,
 url_redirect varchar(100) not null,
 created_at timestamp with time zone not null default now(),
 updated_at timestamp with time zone null
);


CREATE TABLE invoices_det (
 id serial primary key,
 invoice_id integer not null,
 product_code varchar(30) not null,
 quantity numeric(18,2) not null default 0,
 amount numeric(18,2) not null default 0
);


alter table invoices
   add constraint fk_invoices_businesses foreign key (business_id)
      references businesses (id)
      on delete restrict on update restrict;
	  
alter table invoices_det
   add constraint fk_invoices_det_invoices foreign key (invoice_id)
      references invoices (id)
      on delete restrict on update restrict;	  


create unique index idx_cn_businesses on businesses (
 api_id
);

create unique index idx_cn_invoices on invoices (
 business_id,
 payment_address,
 invoice_date,	
 amount	
);

create unique index idx_cn_invoices_det on invoices_det (
 invoice_id,
 product_code	
);
