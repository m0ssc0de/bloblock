use http::method;
use reqwest::blocking::get;

mod download;
mod insert;
pub struct Blob {
    account: String,
    key: String,
    container: String,
}

impl Blob {
    pub fn new(account: String, key: String, container: String) -> Self {
        return Self {
            account,
            key,
            container,
        };
    }
}

enum Actions {
    Download,
    Insert,
}

fn prepare_to_sign(
    account: &str,
    container: &str,
    obj: &str,
    action: Actions,
    time_str: &str,
    conten_length: usize,
) -> String {
    let string_to_sign = {
        let content_encoding = "";
        let content_language = "";
        let content_length = {
            if conten_length == 0 {
                String::from("")
            } else {
                conten_length.to_string()
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
        let version_value = "2015-02-21";
        let (verb, canonicalized_headers) = match action {
            Actions::Download => {
                let verb = http::Method::GET.to_string();
                let canonicalized_headers =
                    format!("x-ms-date:{}\nx-ms-version:{}", time_str, version_value);
                (verb, canonicalized_headers)
            }
            Actions::Insert => {
                let verb = http::Method::PUT.to_string();
                let canonicalized_headers = format!(
                    "x-ms-blob-type:{}\nx-ms-date:{}\nx-ms-version:{}",
                    "BlockBlob", time_str, version_value
                );
                (verb, canonicalized_headers)
            }
        };
        let canonicalized_resource = format!("/{}/{}/{}", account, container, obj);
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
    };
    string_to_sign
}
