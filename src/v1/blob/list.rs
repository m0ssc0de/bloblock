use anyhow::{anyhow, Context, Error};
use http::HeaderValue;
use serde_xml_rs::from_str;

impl<'a> super::Blob<'a> {
    pub fn list(
        &self,
        file_name: &str,
        timefmt: &str,
    ) -> Result<http::Request<std::io::Empty>, Error> {
        let action = super::Actions::List;
        let now = timefmt;

        let mut req_builder = http::Request::builder();
        let formatedkey = format!(
            "SharedKey {}:{}",
            &self.account,
            self.sign(&action, file_name, timefmt, 0)?
        );
        let mut uri = self.container_uri();
        uri.push_str("?restype=container&comp=list");
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
    pub fn parse_list_body(s: &str) -> Result<EnumerationResults, Error> {
        match from_str(s) {
            Ok(d) => Ok(d),
            Err(e) => Err(anyhow!("failed to parse list action body. {}", e)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct EnumerationResults {
    #[serde(rename = "Blobs")]
    blobs: Blobs,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Blobs {
    #[serde(rename = "Blob")]
    blob: Vec<Blob>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Blob {
    // #[serde(rename(serialize = "Name", deserialize = "Name"))]
    #[serde(rename = "Name")]
    name: String,
    // #[serde(rename(serialize = "Properties", deserialize = "Properties"))]
    #[serde(rename = "Properties")]
    properties: Properties,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Properties {
    // #[serde(rename(serialize = "Last-Modified", deserialize = "Last-Modified"))]
    #[serde(rename = "Last-Modified")]
    last_modified: String,
    // #[serde(rename(serialize = "Content-Length", deserialize = "Content-Length"))]
    #[serde(rename = "Content-Length")]
    content_length: usize,
    // #[serde(rename(serialize = "Content-MD5", deserialize = "Content-MD5"))]
    #[serde(rename = "Content-MD5")]
    content_md5: String,
}
