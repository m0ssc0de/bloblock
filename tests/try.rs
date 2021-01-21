use bloblock::blob;
use std::env;

#[test]
fn haha() {
    let account = env::var("STORAGE_ACCOUNT").expect("failed read STORAGE_ACCOUNT from env");
    let key = env::var("STORAGE_MASTER_KEY").expect("failed read STORAGE_MASTER_KEY from env");
    // the above key must be delete from online before publish
    let request = blob::Blob::download(
        &account,
        &key,
        "justry2",
        "test.txt.txt",
        "Thu, 21 Jan 2021 09:18:22 GMT",
    )
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
}
