#[warn(unused_assignments)]

use crate::settings::CONFIG;
use crate::error_handler::CustomError;
use actix_web::http::header::map::HeaderMap;
use base64::encode_config;
use std::time::SystemTime;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, errors::ErrorKind};
use domain::modelsext::Claims;

//use actix_web::FromRequest;

pub struct JWT(pub Claims);

//impl FromRequest for JWT {
//    todo!();
//}

pub async fn verificate_token(headers: &HeaderMap) -> Result<i32, CustomError> {
   let header_value = headers.get("Authorization");
   let token = match header_value {
      Some(value) => {
            let val = value.to_str().unwrap_or_default().to_string();
            if val.starts_with("Bearer ") {
                let token = val.replace("Bearer ","");
                Some(token.to_string())
            } else {
                None
            }
      }
      _ => None,
   };

   match &token {
      Some(token) => {
          // Imprimir el token en la consola para verificar
          println!("Received token: {}", token);

          let secret = CONFIG.jwt.jwt_secret.clone();

          match validate_token(&token, &secret) {
              Ok(claims) => {
                  // Verificar otras condiciones según tus necesidades
                  let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as usize;
                  if claims.exp <= current_time {
                      Err(CustomError::new(401, "Not authorizated".to_string()))
                  } else { 
                      // Puedes agregar más validaciones aquí
                      Ok(claims.sub.trim().parse().expect("error business id!"))
                  }
              }
              Err(_) => Err(CustomError::new(401, "Not authorizated".to_string()))
          }
      }
      None => Err(CustomError::new(401, "Not authorizated".to_string()))
  }

}

fn validate_token(token: &str, secret: &str) -> Result<Claims, CustomError> {
      // Configurar las opciones de validación directamente en Validation
      let mut validation = Validation::new(Algorithm::HS256);
      validation.leeway = 10;  // Ajuste de tiempo permitido para tiempos de expiración y no antes
      validation.validate_exp = true;  // Validar tiempo de expiración (exp)
      validation.validate_nbf = true;  // Validar tiempo antes de inicio (nbf)
      validation.validate_aud = false;  // Validar audiencia (aud)
  
      let decoding_key = DecodingKey::from_secret(secret.as_ref());
  
      let key_base64: String = encode_config(secret.as_bytes(), base64::STANDARD);
  
      println!("Decoding Key (Base64): {}", key_base64);
      println!("Token: {}", token);
      println!("Secret: {}", secret);
  
      match decode::<Claims>(token, &decoding_key, &validation) {
          Ok(token_data) => {
              // Imprimir los valores de iss y aud
              println!("Issuer (iss): {}", token_data.claims.iss);
              println!("Subject (sub): {}", token_data.claims.sub);
              println!("Audience (aud): {}", token_data.claims.aud);
  
              // Verificar que el campo exp sea mayor que el tiempo actual
              let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as usize;
  
              println!("Tiempo del Token: {}", token_data.claims.exp);
  
              if token_data.claims.exp <= current_time {
                  return Err(CustomError::new(401, "Not authorizated".to_string()));
              }
  
              // Si es válido, devolver los claims
              Ok(token_data.claims)
          }
          Err(err) => {
              // Manejar el error de decodificación
              match err.kind() {
                  ErrorKind::ExpiredSignature => {
                      println!("Error decoding token: Token has expired");
                      return Err(CustomError::new(401, "Not authorizated".to_string()));
                  }
                  _ => {
                      println!("Error decoding token: {:?}", err);
                      return Err(CustomError::new(500, "Internal Server Error".to_string()));
                  }
              }
          }
      }
  }