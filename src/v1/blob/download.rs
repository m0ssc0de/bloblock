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
        let string_to_sign = prepare_to_sign(
            &self.account,
            &self.container,
            file_name,
            super::Actions::Download,
            timefmt,
            0,
        );
        let sign = hmacsha256(&self.key, &string_to_sign)?;
        let uri = format!("{}{}", self.uri, file_name);

        let mut req_builder = http::Request::builder();
        let formatedkey = format!("SharedKey {}:{}", &self.account, sign);
        let hm = req_builder.headers_mut().context("context")?;
        hm.insert("Authorization", HeaderValue::from_str(&formatedkey)?);
        hm.insert("x-ms-date", HeaderValue::from_str(&now)?);
        hm.insert("x-ms-version", HeaderValue::from_str(&version_value)?);
        let request = req_builder.method("GET").uri(uri).body(std::io::empty())?;
        Ok(request)
    }
}
