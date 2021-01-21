use anyhow::Error;
use base64::{decode, encode};
use hmac::{Hmac, Mac, NewMac};
// use http::HeaderValue;
use sha2::Sha256;
type HmacSha256 = Hmac<Sha256>;

pub fn hmacsha256(key: &str, string_to_sign: &str) -> Result<String, Error> {
    let mut mac = HmacSha256::new_varkey(&decode(key)?[..]).expect("HMAC can take key of any size"); //(?)
    mac.update(&string_to_sign.to_string().into_bytes()[..]);
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    Ok(encode(code_bytes))
}
