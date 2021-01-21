use anyhow::Error;
mod download;
mod insert;
pub struct Blob<'a> {
    account: &'a str,
    key: &'a str,
    container: &'a str,
}

impl<'a> Blob<'a> {
    pub fn new(account: &'a str, key: &'a str, container: &'a str) -> Self {
        return Self {
            account,
            key,
            container,
        };
    }
    fn uri(&self, file_name: &str) -> String {
        format!(
            "https://{}.blob.core.windows.net/{}/{}",
            self.account, self.container, file_name
        )
    }
    fn sign(
        &self,
        action: Actions,
        file_name: &str,
        time_str: &str,
        content_length: usize,
    ) -> Result<String, Error> {
        let string_to_sign = prepare_to_sign(
            self.account,
            self.container,
            file_name,
            action,
            time_str,
            content_length,
        );

        Ok(crate::sign::hmacsha256(self.key, &string_to_sign)?)
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
    content_length: usize,
) -> String {
    let string_to_sign = {
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
