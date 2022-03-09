use crate::keys::{PRIVATE_KEY_1, PRIVATE_KEY_2, PUBLIC_KEY_1, PUBLIC_KEY_2};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

lazy_static! {
  ///
  static ref ENCODING_KEY_1: EncodingKey = EncodingKey::from_rsa_pem(PRIVATE_KEY_1).unwrap();
  ///
  static ref DECODING_KEY_1: DecodingKey = DecodingKey::from_rsa_pem(PUBLIC_KEY_1).unwrap();
  ///
  static ref ENCODING_KEY_2: EncodingKey = EncodingKey::from_rsa_pem(PRIVATE_KEY_2).unwrap();
  ///
  static ref DECODING_KEY_2: DecodingKey = DecodingKey::from_rsa_pem(PUBLIC_KEY_2).unwrap();
  ///
  static ref VALIDATION: Validation = initialize_validation();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  sub: String,
  exp: usize,
}

/// Encodes JWT token.
pub fn encode_token(sub: &str, seconds: usize) -> String {
  let header = Header::new(Algorithm::RS512);
  let timestamp = (OffsetDateTime::now_utc() + Duration::seconds(seconds as i64)).unix_timestamp() as usize;
  let claims = Claims {
    sub: sub.to_string(),
    exp: timestamp,
  };
  let token = encode(&header, &claims, &ENCODING_KEY_1).unwrap();
  token
}

/// Decodes JWT token with validation.
pub fn decode_token(token: &str) -> bool {
  match decode::<Claims>(token, &DECODING_KEY_1, &VALIDATION) {
    Ok(_) => true,
    Err(_) => false,
  }
}

fn initialize_validation() -> Validation {
  let mut validation = Validation::new(Algorithm::RS512);
  validation.leeway = 0;
  validation.validate_exp = true;
  validation
}
