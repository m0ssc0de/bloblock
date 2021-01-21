use anyhow::{Context, Error};
use http::HeaderValue;

impl<'a> super::Blob<'a> {
    pub fn download(
        &self,
        file_name: &str,
        timefmt: &str,
    ) -> Result<http::Request<std::io::Empty>, Error> {
        let now = timefmt;

        let mut req_builder = http::Request::builder();
        let formatedkey = format!(
            "SharedKey {}:{}",
            &self.account,
            self.sign(super::Actions::Download, file_name, timefmt, 0)?
        );
        let hm = req_builder.headers_mut().context("context")?;
        hm.insert("Authorization", HeaderValue::from_str(&formatedkey)?);
        hm.insert("x-ms-date", HeaderValue::from_str(&now)?);
        hm.insert("x-ms-version", HeaderValue::from_str(&self.version_value)?);
        hm.insert("x-ms-blob-type", HeaderValue::from_str("BlockBlob")?);
        let request = req_builder
            .method("GET")
            .uri(self.uri(file_name))
            .body(std::io::empty())?;
        Ok(request)
    }
}
