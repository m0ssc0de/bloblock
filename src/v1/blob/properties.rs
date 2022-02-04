use anyhow::{Context, Error};
use http::HeaderValue;
use http::Uri;
use std::convert::TryFrom;
use std::str::FromStr;

impl<B> TryFrom<http::Response<B>> for super::PropertiesResponse {
    type Error = Error;
    fn try_from(response: http::Response<B>) -> Result<Self, Error> {
        Ok(Self {
            last_modified: response
                .headers()
                .get("Last-Modified")
                .context("failed to read Last-Modified in headers")?
                .to_str()?
                .to_owned(),
        })
    }
}

impl super::Blob {
    pub fn properties(
        &self,
        file_name: &str,
        timefmt: &str,
    ) -> Result<http::Request<std::io::Empty>, Error> {
        let action = super::Actions::Properties;
        let now = timefmt;

        let mut req_builder = http::Request::builder();
        let mut uri = self.container_uri();
        uri.push('/');
        uri.push_str(file_name);
        let sign = self.sign(
            &super::Actions::Properties,
            Uri::from_str(&uri)?.path(),
            timefmt,
            0,
        );
        let formatedkey = format!(
            "SharedKey {}:{}",
            &self.account,
            sign?,
            // self.sign(&super::Actions::Properties, file_name, timefmt, 0)?
        );
        let hm = req_builder.headers_mut().context("context")?;
        hm.insert("Authorization", HeaderValue::from_str(&formatedkey)?);
        hm.insert("x-ms-date", HeaderValue::from_str(now)?);
        hm.insert("x-ms-version", HeaderValue::from_str(&self.version_value)?);
        let request = req_builder
            .method(http::Method::from(&action))
            .uri(uri)
            .body(std::io::empty())?;
        Ok(request)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_properties() -> Result<(), Error> {
        let account = "t4acc";
        let key =
            "qmVhW8/URPhEpUCQ+iV62m3xGysIArbXw/SNSLE2oCPgRuVlw2Bee4nKlrQsAYgVycoOI201aWheGvarJyzJ/g==";
        let container = "justry2";
        let file_name = "test.txt.txt";
        let download_time = "Thu, 21 Jan 2021 13:36:40 GMT";

        let instance = crate::blob::Blob::new(account, key, container, false);
        let left = instance.properties(file_name, download_time).unwrap();

        // right value
        let right_uri = "https://t4acc.blob.core.windows.net/justry2/test.txt.txt";

        let mut req_builder = http::Request::builder();
        let hm = req_builder.headers_mut().unwrap();
        hm.insert(
            "Authorization",
            HeaderValue::from_str("SharedKey t4acc:UlhoyBhxiZ+dWWCeAEYU+QwXXCeNxDdEsyCB9MM1hZA=")?,
        );
        hm.insert(
            "x-ms-date",
            HeaderValue::from_str("Thu, 21 Jan 2021 13:36:40 GMT")?,
        );
        hm.insert("x-ms-version", HeaderValue::from_str("2015-02-21")?);

        let right = req_builder
            .method(http::Method::HEAD)
            .uri(right_uri)
            .body(std::io::empty())?;

        assert_eq!(left.uri(), right.uri());
        assert_eq!(left.method(), right.method());
        assert_eq!(left.headers(), right.headers());

        Ok(())
    }
}
