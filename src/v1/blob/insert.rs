use crate::sign::hmacsha256;
use anyhow::{Context, Error};
use http::HeaderValue;

use super::prepare_to_sign;

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

        let string_to_sign = prepare_to_sign(
            account,
            container,
            file_name,
            super::Actions::Insert,
            timefmt,
            source.len(),
        );

        let sign = hmacsha256(key, &string_to_sign)?;
        let formatedkey = format!("SharedKey {}:{}", account, sign);

        let uri = format!(
            "https://{}.blob.core.windows.net/{}/{}",
            account, container, file_name
        );
        //
        let mut req_builder = http::Request::builder();
        let hm = req_builder.headers_mut().context("context")?;
        hm.insert("Authorization", HeaderValue::from_str(&formatedkey)?);
        hm.insert("x-ms-date", HeaderValue::from_str(&now)?);
        hm.insert("x-ms-version", HeaderValue::from_str(&version_value)?);
        hm.insert("x-ms-blob-type", HeaderValue::from_str("BlockBlob")?);
        let request = req_builder.method("PUT").uri(uri).body(source)?;
        Ok(request)
    }
}
