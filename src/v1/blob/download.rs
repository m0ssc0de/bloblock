use anyhow::{Context, Error};
use http::HeaderValue;
use http::Uri;

impl<'a> super::Blob<'a> {
    pub fn download(
        &self,
        file_name: &str,
        timefmt: &str,
    ) -> Result<http::Request<std::io::Empty>, Error> {
        let action = super::Actions::Download;
        let now = timefmt;

        let mut req_builder = http::Request::builder();
        let mut uri = self.container_uri();
        uri.push('/');
        uri.push_str(file_name);
        let formatedkey = format!(
            "SharedKey {}:{}",
            &self.account,
            self.sign(
                &action,
                Uri::from_maybe_shared(uri.clone())?.path(),
                timefmt,
                0
            )?
        );
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
#[test]
fn test_download() -> Result<(), Error> {
    let account = "t4acc";
    let key =
        "qmVhW8/URPhEpUCQ+iV62m3xGysIArbXw/SNSLE2oCPgRuVlw2Bee4nKlrQsAYgVycoOI201aWheGvarJyzJ/g==";
    let container = "justry2";
    let file_name = "test.txt.txt";
    let download_time = "Thu, 21 Jan 2021 13:36:40 GMT";

    let instance = crate::blob::Blob::new(account, key, container);
    let left = instance.download(file_name, download_time).unwrap();

    // right value
    let right_uri = "https://t4acc.blob.core.windows.net/justry2/test.txt.txt";

    let mut req_builder = http::Request::builder();
    let hm = req_builder.headers_mut().unwrap();
    hm.insert(
        "Authorization",
        HeaderValue::from_str("SharedKey t4acc:PDOHjdh5rHgB1HoWgJ7sZx/VLtBBOazmsneLFIU0Fcc=")?,
    );
    hm.insert(
        "x-ms-date",
        HeaderValue::from_str("Thu, 21 Jan 2021 13:36:40 GMT")?,
    );
    hm.insert("x-ms-version", HeaderValue::from_str("2015-02-21")?);
    hm.insert("x-ms-blob-type", HeaderValue::from_str("BlockBlob")?);

    let right = req_builder
        .method(http::Method::GET)
        .uri(right_uri)
        .body(std::io::empty())?;

    assert_eq!(left.uri(), right.uri());
    assert_eq!(left.method(), right.method());
    assert_eq!(left.headers(), right.headers());

    Ok(())
}
