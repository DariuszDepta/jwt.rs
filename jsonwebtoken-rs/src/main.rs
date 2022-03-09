#![feature(test)]

use crate::auth::{decode_token, encode_token};
use std::thread::sleep;
use std::time::Duration;

extern crate jsonwebtoken;
extern crate lazy_static;
extern crate serde;
extern crate serde_derive;

mod auth;
#[cfg(test)]
mod benches;
mod keys;

fn main() {
  let encoded = encode_token("subject", 2);
  println!("encoded: {}", encoded);
  sleep(Duration::new(1, 0));
  let decoded = decode_token(&encoded);
  println!("decoded: {}", decoded);
}
