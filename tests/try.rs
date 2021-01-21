use bloblock::blob;
// use bytes::Bytes;
// use http::response;
use std::env;

#[test]
fn haha() {
    let account = env::var("STORAGE_ACCOUNT").expect("failed read STORAGE_ACCOUNT from env");
    let key = env::var("STORAGE_MASTER_KEY").expect("failed read STORAGE_MASTER_KEY from env");
    // the above key must be delete from online before publish
    let instance = blob::Blob::new(&account, &key, "justry2");
    let request = instance
        .download("test.txt.txt", "Thu, 21 Jan 2021 09:18:22 GMT")
        .unwrap();

    let (p, _) = request.into_parts();

    assert_eq!(p.method, http::Method::GET);
    assert_eq!(
        p.uri,
        "https://t4acc.blob.core.windows.net/justry2/test.txt.txt"
    );
    assert_eq!(
        format!("{:?}", p.headers), 
        "{\"authorization\": \"SharedKey t4acc:NgSvaCnTsGKyFmpKEPhoOu2q0Gq/tZlzvVbVkRLN1Yo=\", \"x-ms-date\": \"Thu, 21 Jan 2021 09:18:22 GMT\", \"x-ms-version\": \"2015-02-21\"}"
    );

    // insert
    use chrono::Utc;
    let now = Utc::now().format("%a, %e %b %Y %T GMT").to_string();
    let content = bytes::Bytes::from("hello world");
    let request = blob::Blob::insert(
        &account,
        &key,
        "justry2",
        "test.txt.txt",
        content,
        // "Thu, 21 Jan 2021 10:25:47 GMT",
        &now,
    )
    .unwrap();
    let (p, b) = request.into_parts();
    assert_eq!(p.method, http::Method::PUT);
    assert_eq!(
        p.uri,
        "https://t4acc.blob.core.windows.net/justry2/test.txt.txt"
    );
    // assert_eq!(
    //     format!("{:?}", p.headers),
    //     "{\"authorization\": \"SharedKey t4acc:P5s6MPQMGssba4bnHAq2Ymiuujqj9LlOCQwJziINmyE=\", \"x-ms-date\": \"Thu, 21 Jan 2021 10:25:47 GMT\", \"x-ms-version\": \"2015-02-21\"}"
    // );
    let client = reqwest::blocking::Client::new();
    let response = client
        .put(&p.uri.to_string())
        .headers(p.headers)
        .body(b)
        .send()
        .unwrap();
    assert_eq!(response.text().unwrap(), "");
    // assert_eq!(response.status(), reqwest::StatusCode::OK);
}
