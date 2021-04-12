use anyhow::Error;
use std::fmt;
mod download;
mod insert;
mod list;
mod properties;

pub use list::parse_list_body;
pub use list::EnumerationResults;

pub struct PropertiesResponse {
    pub last_modified: String,
}
pub struct Blob<'a> {
    account: &'a str,
    key: &'a str,
    container: &'a str,
    version_value: String,
    azurite: bool,
}

impl<'a> Blob<'a> {
    pub fn new(account: &'a str, key: &'a str, container: &'a str, azurite: bool) -> Self {
        Self {
            account,
            key,
            container,
            version_value: String::from("2015-02-21"),
            azurite,
        }
    }
    fn container_uri(&self) -> String {
        if self.azurite {
            format!("http://127.0.0.1:10000/{}/{}", self.account, self.container)
        } else {
            format!(
                "https://{}.blob.core.windows.net/{}",
                self.account, self.container
            )
        }
    }
    // fn headers(&self) {}
    fn sign(
        &self,
        action: &Actions,
        path: &str,
        time_str: &str,
        content_length: usize,
    ) -> Result<String, Error> {
        let string_to_sign = prepare_to_sign(
            self.account,
            path,
            action,
            time_str,
            content_length,
            &self.version_value,
        );

        // (
        crate::sign::hmacsha256(self.key, &string_to_sign)
        //     string_to_sign,
        // )
    }
}

impl<'a> fmt::Debug for Blob<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Blob: {:#?}", self)
    }
}

enum Actions {
    Download,
    Insert,
    Properties,
    List,
}

impl From<&Actions> for http::Method {
    fn from(action: &Actions) -> Self {
        match action {
            Actions::Download => http::Method::GET,
            Actions::Insert => http::Method::PUT,
            Actions::Properties => http::Method::HEAD,
            Actions::List => http::Method::GET,
        }
    }
}

fn prepare_to_sign(
    account: &str,
    path: &str,
    action: &Actions,
    time_str: &str,
    content_length: usize,
    version_value: &str,
) -> String {
    {
        let content_encoding = "";
        let content_language = "";
        let content_length = {
            if content_length == 0 {
                String::from("")
            } else {
                content_length.to_string()
            }
        };
        let content_md5 = "";
        let content_type = "";
        let date = "";
        let if_modified_since = "";
        let if_match = "";
        let if_none_match = "";
        let if_unmodified_since = "";
        let range = "";
        let canonicalized_headers = match action {
            Actions::Properties => {
                format!("x-ms-date:{}\nx-ms-version:{}", time_str, version_value)
            }
            _ => format!(
                "x-ms-blob-type:{}\nx-ms-date:{}\nx-ms-version:{}",
                "BlockBlob", time_str, version_value
            ),
        };
        // let canonicalized_headers =
        //     format!("x-ms-date:{}\nx-ms-version:{}", time_str, version_value);
        let verb = http::Method::from(action).to_string();
        let canonicalized_resource = match action {
            Actions::List => format!("/{}{}\ncomp:list\nrestype:container", account, path),
            _ => format!("/{}{}", account, path),
        };
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            verb,
            content_encoding,
            content_language,
            content_length,
            content_md5,
            content_type,
            date,
            if_modified_since,
            if_match,
            if_none_match,
            if_unmodified_since,
            range,
            canonicalized_headers,
            canonicalized_resource,
        )
    }
}

#[test]
fn test_prepare_to_sign() -> Result<(), Error> {
    let left = "PUT\n\n\n11\n\n\n\n\n\n\n\n\nx-ms-blob-type:BlockBlob\nx-ms-date:Tue, 06 Apr 2021 14:08:27 GMT\nx-ms-version:2015-02-21\n/t4acc/ccon/test_bloblock.txt";
    let right = prepare_to_sign(
        "t4acc",
        "/ccon/test_bloblock.txt",
        &Actions::Insert,
        "Tue, 06 Apr 2021 14:08:27 GMT",
        11,
        "2015-02-21",
    );
    assert_eq!(left, right);

    Ok(())
}

#[test]
fn test_sign() -> Result<(), Error> {
    let b = Blob::new(
        "devstoreaccount1",
        "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==",
        "ccon",
        false,
    );
    let right = b
        .sign(
            &Actions::Insert,
            "/ccon/test_bloblock.txt",
            "Tue, 06 Apr 2021 14:08:27 GMT",
            11,
        )
        .unwrap();
    assert_eq!(right, "AqBQs2cXXFB4+G0x3oevuOtxH65IRzA1oIuVvdptRzc=");
    Ok(())
}
