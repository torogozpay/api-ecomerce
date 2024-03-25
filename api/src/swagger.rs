use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use domain::models::Business;
use domain::modelsext::NewInvoice;
use crate::business_handler as business;
use crate::invoice_handler as invoice;

#[derive(OpenApi)]
#[openapi(
    paths(
        business::list_businesses_handler,
        business::list_business_by_id_handler,
        business::list_business_by_api_handler,
        business::create_business_handler,
        business::update_business_handler,
        business::delete_business_handler,
        invoice::list_invoice_handler,
        invoice::save_preorder_handler,
    ),
    components(schemas(Business,NewInvoice))
)]
pub struct ApiDoc;

pub fn init_swagger(config: &mut web::ServiceConfig) {
    let openapi = ApiDoc::openapi();
    config.service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi));
}