# ApiEcomerce - Ecommerce Api from TorogozPay

Api to maintain communication between the Lightning Api and Business Api, as well as the interaction between the Woocommerce Plugin to allow payments in lighning.


## Requirements:

0. You need Rust version 1.75 or higher to compile.


## Install

Clone the repository and then create a new `.env` file based on `.env-sample` file.

```
$ git clone https://github.com/torogozpay/api-ecomerce.git
$ cd api-ecomerce
$ cp .env-sample .env
```

To connect 

we need to set 1 variable in the `.env` file .
RUN_ENV = Development

Possible environment values are
    Development
    Testing
    Production

We need to create an environment configuration file for the established environment, in the /shared/src/config/ directory
If we set "RUN_ENV = Development", we must create the file Development.toml with the following structure.

	[server]
	database_url = "postgres://USER:PASSSWORD@SERVER:PORT/ecommerceapi?sslmode=disable"
	host = "127.0.0.1"
	port = "8282"

	[log]
	level = "debug"

	[openapi]
	swagger = true

	[api]
	api_server= "https://DOMAIN"
	api_user= 
	api_pass= 

	[jwt]
	jwt_secret= 
	jwt_secs= 3600 


### Database

The data is saved in a postgres db. Before start building we need to initialize the database, for this we need to use `diesel_cli`:

```
$ sudo apt update
$ sudo apt install libpq-dev
$ cargo install diesel_cli --no-default-features --features postgres
```

Now we can create the database manually "ecommerceapi", and when starting the api the necessary tables will be created.
Otherwise, execute the following commands from the infrastructure directory

```
diesel setup
diesel migration run
```


## Install dependencies

To compile on Ubuntu/Pop!\_OS, you need to install some dependencies, run the following commands:

```
$ sudo apt update
$ sudo apt install -y cmake build-essential pkg-config libssl-dev
```

## Compile and execute it:

```
$ cargo build --release
$ target/release/api-ecomerce
```