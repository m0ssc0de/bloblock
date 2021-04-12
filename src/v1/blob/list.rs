use anyhow::{anyhow, Context, Error};
use http::HeaderValue;
use http::Uri;
use serde_xml_rs::from_str;
use std::str::FromStr;

impl<'a> super::Blob<'a> {
    pub fn list(&self, timefmt: &str) -> Result<http::Request<std::io::Empty>, Error> {
        let action = super::Actions::List;
        let now = timefmt;

        let mut req_builder = http::Request::builder();
        let mut uri = self.container_uri();
        uri.push_str("?restype=container&comp=list");
        let sign = self.sign(&action, Uri::from_str(&uri)?.path(), timefmt, 0);
        let formatedkey = format!(
            "SharedKey {}:{}",
            &self.account,
            sign?,
            // self.sign(&action, Uri::from_str(&uri)?.path(), timefmt, 0)?
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
fn test_list() -> Result<(), Error> {
    let account = "t4acc";
    let key =
        "qmVhW8/URPhEpUCQ+iV62m3xGysIArbXw/SNSLE2oCPgRuVlw2Bee4nKlrQsAYgVycoOI201aWheGvarJyzJ/g==";
    let container = "justry2";
    let download_time = "Thu, 21 Jan 2021 13:36:40 GMT";

    let instance = crate::blob::Blob::new(account, key, container, false);
    let left = instance.list(download_time).unwrap();

    // right value
    let right_uri = "https://t4acc.blob.core.windows.net/justry2?restype=container&comp=list";

    let mut req_builder = http::Request::builder();
    let hm = req_builder.headers_mut().unwrap();
    hm.insert(
        "Authorization",
        HeaderValue::from_str("SharedKey t4acc:2Ah0usXqANaJH4LbkMdDmVXKfRMl0tn4oGz3TWY0B5Q=")?,
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct EnumerationResults {
    #[serde(rename = "Blobs")]
    pub blobs: Blobs,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Blobs {
    #[serde(rename = "Blob")]
    pub blob: Vec<Blob>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Blob {
    // #[serde(rename(serialize = "Name", deserialize = "Name"))]
    #[serde(rename = "Name")]
    pub name: String,
    // #[serde(rename(serialize = "Properties", deserialize = "Properties"))]
    #[serde(rename = "Properties")]
    pub properties: Properties,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Properties {
    // #[serde(rename(serialize = "Last-Modified", deserialize = "Last-Modified"))]
    #[serde(rename = "Last-Modified")]
    pub last_modified: String,
    // #[serde(rename(serialize = "Content-Length", deserialize = "Content-Length"))]
    #[serde(rename = "Content-Length")]
    pub content_length: usize,
    // #[serde(rename(serialize = "Content-MD5", deserialize = "Content-MD5"))]
    #[serde(rename = "Content-MD5")]
    pub content_md5: String,
}

pub fn parse_list_body(s: &str) -> Result<EnumerationResults, Error> {
    match from_str(s) {
        Ok(d) => Ok(d),
        Err(e) => Err(anyhow!("failed to parse list action body. {}", e)),
    }
}

#[test]
fn test_parse_list_body() {
    let body = r#"
<?xml version="1.0" encoding="utf-8"?>
<EnumerationResults ServiceEndpoint=\"https://t4acc.blob.core.windows.net/\" ContainerName=\"justry2\">
	<Blobs>
		<Blob>
			<Name>test.txt</Name>
			<Properties>
				<Last-Modified>Fri, 22 Jan 2021 13:11:34 GMT</Last-Modified>
				<Etag>0x8D8BED73E8F74B5</Etag>
				<Content-Length>7</Content-Length>
				<Content-Type>text/plain</Content-Type>
				<Content-Encoding />
				<Content-Language />
				<Content-MD5>oUHEeSeSm8LR+20zaiVt9A==</Content-MD5>
				<Cache-Control />
				<Content-Disposition />
				<BlobType>BlockBlob</BlobType>
				<LeaseStatus>unlocked</LeaseStatus>
				<LeaseState>available</LeaseState>
			</Properties>
		</Blob>
		<Blob>
			<Name>test.txt.txt</Name>
			<Properties>
                <Last-Modified>Fri, 22 Jan 2021 04:00:21 GMT</Last-Modified>
				<Etag>0x8D8BE8A3D264D86</Etag>
				<Content-Length>11</Content-Length>
				<Content-Type>application/octet-stream</Content-Type>
				<Content-Encoding />
				<Content-Language />
				<Content-MD5>XrY7u+Ae7tCTyyK7j1rNww==</Content-MD5>
				<Cache-Control />
				<Content-Disposition />
				<BlobType>BlockBlob</BlobType>
				<LeaseStatus>unlocked</LeaseStatus>
				<LeaseState>available</LeaseState>
			</Properties>
		</Blob>
	</Blobs>
	<NextMarker />
</EnumerationResults>
        "#;
    let rep = parse_list_body(body).unwrap();
    let right = EnumerationResults {
        blobs: Blobs {
            blob: vec![
                Blob {
                    name: String::from("test.txt"),
                    properties: Properties {
                        last_modified: String::from("Fri, 22 Jan 2021 13:11:34 GMT"),
                        content_length: 7,
                        content_md5: String::from("oUHEeSeSm8LR+20zaiVt9A=="),
                    },
                },
                Blob {
                    name: String::from("test.txt.txt"),
                    properties: Properties {
                        last_modified: String::from("Fri, 22 Jan 2021 04:00:21 GMT"),
                        content_length: 11,
                        content_md5: String::from("XrY7u+Ae7tCTyyK7j1rNww=="),
                    },
                },
            ],
        },
    };
    assert_eq!(rep, right);
}
