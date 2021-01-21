use anyhow::{Context, Error};
use base64::{decode, encode};
use hmac::{Hmac, Mac, NewMac};
use http::HeaderValue;
use sha2::Sha256;

impl super::Blob {
    pub fn insert(
        account: &str,
        key: &str,
        container: &str,
        file_name: &str,
        source: bytes::Bytes,
        timefmt: &str,
    ) -> Result<http::Request<bytes::Bytes>, Error> {
        let now = timefmt;
        let version_value = "2015-02-21";
        let obj = file_name;

        let string_to_sign = {
            let verb = "PUT"; //
            let content_encoding = "";
            let content_language = "";
            let content_length = source.len(); //
            let content_md5 = "";
            // let content_type = "text/plain";
            let content_type = "";
            let date = "";
            let if_modified_since = "";
            let if_match = "";
            let if_none_match = "";
            let if_unmodified_since = "";
            let range = "";
            let canonicalized_headers = format!(
                "x-ms-blob-type:{}\nx-ms-date:{}\nx-ms-version:{}",
                "BlockBlob", now, version_value
            );
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
        println!("to sign :{}", string_to_sign);

        type HmacSha256 = Hmac<Sha256>;
        let mut mac =
            HmacSha256::new_varkey(&decode(key)?[..]).expect("HMAC can take key of any size"); //(?)
        mac.update(&string_to_sign.into_bytes()[..]);
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        println!("Hello, world! {}", encode(code_bytes));
        let uri = format!(
            "https://{}.blob.core.windows.net/{}/{}",
            account, container, file_name
        );
        //
        let mut req_builder = http::Request::builder();
        let formatedkey = format!("SharedKey {}:{}", account, encode(code_bytes));
        // let mut hm = http::HeaderMap::new();
        // req_builder.headers_mut().
        let hm = req_builder.headers_mut().context("context")?;
        hm.insert("Authorization", HeaderValue::from_str(&formatedkey)?);
        hm.insert("x-ms-date", HeaderValue::from_str(&now)?);
        hm.insert("x-ms-version", HeaderValue::from_str(&version_value)?);
        hm.insert("x-ms-blob-type", HeaderValue::from_str("BlockBlob")?);
        let request = req_builder.method("PUT").uri(uri).body(source)?;
        Ok(request)
    }
}
