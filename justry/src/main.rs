use base64::{decode, encode};
use chrono::Utc;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use std::env;

fn main() {
    let account = env::var("STORAGE_ACCOUNT").expect("failed read STORAGE_ACCOUNT from env");
    let key = env::var("STORAGE_MASTER_KEY").expect("failed read STORAGE_MASTER_KEY from env");

    let container = "justry2";
    let now = Utc::now().format("%a, %d %b %Y %T GMT").to_string();
    let obj = "test.txt.txt";
    let path_to_sign = format!("/{}/{}", container, obj);

    let StringToSign = {
        let VERB = "GET";
        let ContentEncoding = "";
        let ContentLanguage = "";
        let ContentLength = "";
        let ContentMD5 = "";
        let ContentType = "";
        let Date = "";
        let IfModifiedSince = "";
        let IfMatch = "";
        let IfNoneMatch = "";
        let IfUnmodifiedSince = "";
        let Range = "";
        let CanonicalizedHeaders = format!("x-ms-date:{}\nx-ms-version:2015-02-21", now);
        // let CanonicalizedResource = format!("/{}/{}/{}", account, container, obj);
        let CanonicalizedResource = format!("/{}{}", account, path_to_sign);
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            VERB,
            ContentEncoding,
            ContentLanguage,
            ContentLength,
            ContentMD5,
            ContentType,
            Date,
            IfModifiedSince,
            IfMatch,
            IfNoneMatch,
            IfUnmodifiedSince,
            Range,
            CanonicalizedHeaders,
            CanonicalizedResource,
        )
    };

    type HmacSha256 = Hmac<Sha256>;
    let mut mac =
        HmacSha256::new_varkey(&decode(key).unwrap()[..]).expect("HMAC can take key of any size");
    mac.update(&StringToSign.into_bytes()[..]);
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    println!("timenow: {}\n", now);
    println!("string to sign: ->{}<-", StringToSign);
    println!("Hello, world! {}", encode(code_bytes));
}
