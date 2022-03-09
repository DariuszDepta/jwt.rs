extern crate test;

use crate::{decode_token, encode_token};
use test::Bencher;

#[bench]
fn decoding_token(b: &mut Bencher) {
  let encoded = encode_token("subject", 2);
  b.iter(|| decode_token(&encoded));
}
