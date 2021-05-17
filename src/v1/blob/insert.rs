use std::str::FromStr;

use anyhow::{Context, Error};
use http::HeaderValue;
use http::Uri;

impl super::Blob {
    pub fn insert(
        &self,
        file_name: &str,
        source: bytes::Bytes,
        timefmt: &str,
    ) -> Result<http::Request<bytes::Bytes>, Error> {
        let action = super::Actions::Insert;
        let now = timefmt;

        let mut uri = self.container_uri();
        uri.push('/');
        uri.push_str(file_name);
        let sign = self.sign(&action, Uri::from_str(&uri)?.path(), timefmt, source.len());
        let formatedkey = format!("SharedKey {}:{}", self.account, sign?);
        let mut req_builder = http::Request::builder();
        let hm = req_builder.headers_mut().context("context")?;
        hm.insert("Authorization", HeaderValue::from_str(&formatedkey)?);
        hm.insert("x-ms-date", HeaderValue::from_str(&now)?);
        hm.insert("x-ms-version", HeaderValue::from_str(&self.version_value)?);
        hm.insert("x-ms-blob-type", HeaderValue::from_str("BlockBlob")?);
        let request = req_builder
            .method(http::Method::from(&action))
            .uri(uri)
            .body(source)?;
        Ok(request)
    }
}

#[test]
fn test_insert() -> Result<(), Error> {
    let account = "t4acc";
    let key =
        "qmVhW8/URPhEpUCQ+iV62m3xGysIArbXw/SNSLE2oCPgRuVlw2Bee4nKlrQsAYgVycoOI201aWheGvarJyzJ/g==";
    let container = "justry2";
    let request_time = "Thu, 21 Jan 2021 13:36:40 GMT";
    let content = bytes::Bytes::from("hello world");

    let instance = crate::blob::Blob::new(account, key, container, false);
    let left = instance
        .insert("test.txt.txt", content, request_time)
        .unwrap();

    // right value
    let right_uri = "https://t4acc.blob.core.windows.net/justry2/test.txt.txt";

    let mut req_builder = http::Request::builder();
    let hm = req_builder.headers_mut().unwrap();
    hm.insert(
        "Authorization",
        HeaderValue::from_str("SharedKey t4acc:itYn9WXDkvL72lk/iiVNG4m5VtJHY//QUX03b78uvZg=")?,
    );
    hm.insert(
        "x-ms-date",
        HeaderValue::from_str("Thu, 21 Jan 2021 13:36:40 GMT")?,
    );
    hm.insert("x-ms-version", HeaderValue::from_str("2015-02-21")?);
    hm.insert("x-ms-blob-type", HeaderValue::from_str("BlockBlob")?);

    let right = req_builder
        .method(http::Method::PUT)
        .uri(right_uri)
        .body(std::io::empty())?;

    assert_eq!(left.uri(), right.uri());
    assert_eq!(left.method(), right.method());
    assert_eq!(left.headers(), right.headers());

    Ok(())
}
