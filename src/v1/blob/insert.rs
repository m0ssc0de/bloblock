use anyhow::{Context, Error};
use http::HeaderValue;

impl<'a> super::Blob<'a> {
    pub fn insert(
        // account: &str,
        // key: &str,
        // container: &str,
        &self,
        file_name: &str,
        source: bytes::Bytes,
        timefmt: &str,
    ) -> Result<http::Request<bytes::Bytes>, Error> {
        let now = timefmt;
        let version_value = "2015-02-21";

        let formatedkey = format!(
            "SharedKey {}:{}",
            self.account,
            self.sign(super::Actions::Insert, file_name, timefmt, source.len())?
        );

        let mut req_builder = http::Request::builder();
        let hm = req_builder.headers_mut().context("context")?;
        hm.insert("Authorization", HeaderValue::from_str(&formatedkey)?);
        hm.insert("x-ms-date", HeaderValue::from_str(&now)?);
        hm.insert("x-ms-version", HeaderValue::from_str(&version_value)?);
        hm.insert("x-ms-blob-type", HeaderValue::from_str("BlockBlob")?);
        let request = req_builder
            .method("PUT")
            .uri(self.uri(file_name))
            .body(source)?;
        Ok(request)
    }
}
