use actix_web::{post, web, HttpResponse};
use jsonwebtoken::{encode, Header, EncodingKey, Algorithm};
use serde::{Serialize, Deserialize};
use std::time::SystemTime;
use shared::{error_handler::CustomError,settings::CONFIG};
use application::config_business;
use domain::modelsext::Claims;



// Definir una estructura para representar la respuesta del token
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    access_token: String,
    token_type: &'static str,
}

// Estructura para representar los datos de la solicitud
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestData {
    grant_type: Option<String>,
    client_id: Option<String>,
    client_secret: Option<String>,
    audience: Option<String>,  // Nueva propiedad para la audiencia
}

// Estructura para representar la respuesta de error
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    code: u32,
    message: String,
}

// Endpoint para generar el token
#[post("/generateAccessToken")]
pub async fn generate_access_token_handler(data: web::Json<RequestData>) -> Result<HttpResponse, CustomError> {

    // Verificar la presencia de grant_type, client_id y client_secret
    if data.grant_type.is_none() || data.grant_type.as_ref().unwrap().trim().is_empty() {
        return Err(CustomError::new(10, "grant_type is required and must not be empty".to_string()));
    }

    if data.client_id.is_none() || data.client_id.as_ref().unwrap().trim().is_empty() {
        return Err(CustomError::new(20, "client_id is required and must not be empty".to_string()));
    }

    if data.client_secret.is_none() || data.client_secret.as_ref().unwrap().trim().is_empty() {
        return Err(CustomError::new(30, "client_secret is required and must not be empty".to_string()));
    }

    // Verificar la presencia y no nulidad de la audiencia
    if data.audience.is_none() || data.audience.as_ref().unwrap().trim().is_empty() {
        return Err(CustomError::new(40, "audience is required and must not be empty".to_string()));
    }

    match config_business(data.client_id.clone().expect("client_id"), data.client_secret.clone().expect("client_secret")).await {
        Ok(_config) => {      
        
            // Obtener los valores de la solicitud
            let client_id = data.client_id.as_ref().unwrap();
            let client_secret = data.client_secret.as_ref().unwrap();
            let audience = data.audience.as_ref().unwrap();
        
            // Verificar las credenciales del cliente
            if client_id != &_config.api_id || client_secret != &_config.api_secret {
                return Err(CustomError::new(50, "Invalid client credentials".to_string()));
            }
        
            // Generar los claims del token
            let claims = Claims {
                iss: _config.api_id.clone(), //"your_issuer".to_owned(),
                sub: _config.id.to_string().clone(), //"your_subject".to_owned(),
                aud: audience.clone(),  // Utilizar el valor de audiencia del cuerpo del JSON
                exp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as usize + CONFIG.jwt.jwt_secs.clone(),
                iat: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as usize,
            };
        
            // Generar el token utilizando la clave secreta de la variable de entorno
            // Crear una clave de codificación
            let secret_key = CONFIG.jwt.jwt_secret.clone();
            
            let encoding_key = EncodingKey::from_secret(secret_key.as_ref());
        
            let token_data = TokenResponse {
                access_token: encode(
                    &Header::new(Algorithm::HS256),
                    &claims,
                    &encoding_key,
                )
                .map_err(|e| CustomError::new(60, "Failed to generate access token {}".to_string() + &e.to_string()))?,
                token_type: "Bearer",
            };
        
            Ok(HttpResponse::Ok().json(token_data))
        

        },
        Err(_) => Err(CustomError::new(401, "Not authorizated".to_string()))
    } 
   
}