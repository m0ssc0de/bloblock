use anyhow::{Context, Error};
use http::HeaderValue;

impl<'a> super::Blob<'a> {
    pub fn download(
        &self,
        file_name: &str,
        timefmt: &str,
    ) -> Result<http::Request<std::io::Empty>, Error> {
        let action = super::Actions::Download;
        let now = timefmt;

        let mut req_builder = http::Request::builder();
        let formatedkey = format!(
            "SharedKey {}:{}",
            &self.account,
            self.sign(&action, file_name, timefmt, 0)?
        );
        let mut uri = self.container_uri();
        uri.push('/');
        uri.push_str(file_name);
        let hm = req_builder.headers_mut().context("context")?;
        hm.insert("Authorization", HeaderValue::from_str(&formatedkey)?);
        hm.insert("x-ms-date", HeaderValue::from_str(&now)?);
        hm.insert("x-ms-version", HeaderValue::from_str(&self.version_value)?);
        hm.insert("x-ms-blob-type", HeaderValue::from_str("BlockBlob")?);
        let request = req_builder
            .method(http::Method::from(&action))
            .uri(uri)
            .body(std::io::empty())?;
        Ok(request)
    }
}
