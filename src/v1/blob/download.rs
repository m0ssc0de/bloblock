use crate::sign::hmacsha256;
use anyhow::{Context, Error};
use http::HeaderValue;

use super::prepare_to_sign;
impl<'a> super::Blob<'a> {
    pub fn download(
        &self,
        // account: &str,
        // key: &str,
        // container: &str,
        file_name: &str,
        timefmt: &str,
    ) -> Result<http::Request<std::io::Empty>, Error> {
        // let now = Utc::now().format("%a, %e %b %Y %T GMT").to_string();
        let now = timefmt;
        let version_value = "2015-02-21";

        let mut req_builder = http::Request::builder();
        let formatedkey = format!(
            "SharedKey {}:{}",
            &self.account,
            self.sign(super::Actions::Download, file_name, timefmt, 0)?
        );
        let hm = req_builder.headers_mut().context("context")?;
        hm.insert("Authorization", HeaderValue::from_str(&formatedkey)?);
        hm.insert("x-ms-date", HeaderValue::from_str(&now)?);
        hm.insert("x-ms-version", HeaderValue::from_str(&version_value)?);
        let request = req_builder
            .method("GET")
            .uri(self.uri(file_name))
            .body(std::io::empty())?;
        Ok(request)
    }
}
