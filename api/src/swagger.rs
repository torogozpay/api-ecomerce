use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi,Config};
use utoipa::openapi::security::{HttpAuthScheme,Http,SecurityScheme};

use domain::models::{Business,NewBusiness,NewUser};
use domain::modelsext::{NewInvoice,NewInvoiceDet,BusinessCreated,RequestData,CurrencyFilters,InvoiceFilters,OrderFilters};
use crate::business_handler as business;
use crate::invoice_handler as invoice;
use crate::jwt_handler as jwt;
use crate::test_handler as test;
use crate::utils::response;


#[derive(OpenApi)]
#[openapi(
    paths(
        jwt::generate_access_token_handler,
        test::get_price_bitcoin_handler,
        business::list_businesses_handler,
        business::list_business_by_id_handler,
        business::list_business_by_api_handler,
        business::create_business_handler,
        business::update_business_handler,
        business::delete_business_handler,
        business::generate_data_api_handler,
        invoice::list_order_handler,
        invoice::list_invoice_handler,
        invoice::save_preorder_handler,
    ),
    components(schemas(NewUser,Business,NewBusiness,BusinessCreated,NewInvoice,NewInvoiceDet,
                       RequestData,CurrencyFilters,InvoiceFilters,OrderFilters,
                       response::BusinessesResponse,response::BusinessResponse,
                       response::InvoicesResponse,response::InvoiceResponse,
                       response::DeleteResponse,response::ErrorResponse))
)]
pub struct ApiDoc;

pub fn init_swagger(config: &mut web::ServiceConfig) {
    let mut openapi = ApiDoc::openapi();

    let components: &mut utoipa::openapi::Components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
    components.add_security_scheme(
        "bearerAuth",
        SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
    );

    // Assuming you have a service method for registering routes
    let swagger = SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", openapi)
                    .config(Config::default().try_it_out_enabled(false).filter(false)); 

    config.service(swagger);
}