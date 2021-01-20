use anyhow::{Context, Error};
use base64::{decode, encode};
use chrono::Utc;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
impl super::Blob {
    pub fn download(
        account: &str,
        key: &str,
        container: &str,
        file_name: &str,
    ) -> Result<http::Request<std::io::Empty>, Error> {
        // head sign
        // let container = "justry2";
        let now = Utc::now().format("%a, %e %b %Y %T GMT").to_string();
        println!("{}\n", now);
        // let obj = "test.txt.txt";
        let obj = file_name;
        let string_to_sign = {
            let verb = "GET";
            let content_encoding = "";
            let content_language = "";
            let content_length = "";
            let content_md5 = "";
            let content_type = "";
            let date = "";
            let if_modified_since = "";
            let if_match = "";
            let if_none_match = "";
            let if_unmodified_since = "";
            let range = "";
            let canonicalized_headers = format!("x-ms-date:{}\nx-ms-version:2015-02-21", now);
            let canonicalized_resource = format!("/{}/{}/{}", account, container, obj);
            format!(
                "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
                verb,
                content_encoding,
                content_language,
                content_length,
                content_md5,
                content_type,
                date,
                if_modified_since,
                if_match,
                if_none_match,
                if_unmodified_since,
                range,
                canonicalized_headers,
                canonicalized_resource,
            )
        };
        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_varkey(&decode(key).unwrap()[..])
            .expect("HMAC can take key of any size");
        mac.update(&string_to_sign.into_bytes()[..]);
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        println!("Hello, world! {}", encode(code_bytes));
        //
        let req_builder = http::Request::builder();
        Ok(req_builder
            .method("GET")
            .uri("https://www.google.com")
            .body(std::io::empty())?)
    }
}
