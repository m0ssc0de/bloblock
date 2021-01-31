use anyhow::{anyhow, Error};
use bloblock::blob;
use http::response;
use std::env;

#[test]
fn haha() {
    let account = env::var("STORAGE_ACCOUNT").expect("failed read STORAGE_ACCOUNT from env");
    let key = env::var("STORAGE_MASTER_KEY").expect("failed read STORAGE_MASTER_KEY from env");
    // the above key must be delete from online before publish
    let instance = blob::Blob::new(&account, &key, "justry2");
    let request = instance
        .download("test.txt.txt", "Thu, 21 Jan 2021 13:36:40 GMT")
        .unwrap();

    let (p, _) = request.into_parts();

    assert_eq!(p.method, http::Method::GET);
    assert_eq!(
        p.uri,
        "https://t4acc.blob.core.windows.net/justry2/test.txt.txt"
    );
    // assert_eq!(
    //     format!("{:?}", p.headers),
    //     "{\"authorization\": \"SharedKey t4acc:W2F7Wq3pzXtcPHbsAxN8eL9SODwCf4j22+y9QTnQYw4=\", \"x-ms-date\": \"Thu, 21 Jan 2021 13:36:40 GMT\", \"x-ms-version\": \"2015-02-21\", \"x-ms-blob-type\": \"BlockBlob\"}"
    // );

    // // insert
    use chrono::Utc;
    let now = Utc::now().format("%a, %e %b %Y %T GMT").to_string();
    // let content = bytes::Bytes::from("hello world");
    // let request = instance.insert("test.txt.txt", content, &now).unwrap();
    // let (p, b) = request.into_parts();
    // assert_eq!(p.method, http::Method::PUT);
    // assert_eq!(
    //     p.uri,
    //     "https://t4acc.blob.core.windows.net/justry2/test.txt.txt"
    // );
    // let client = reqwest::blocking::Client::new();
    // let response = client
    //     .put(&p.uri.to_string())
    //     .headers(p.headers)
    //     .body(b)
    //     .send()
    //     .unwrap();
    // assert_eq!(response.text().unwrap(), "");
    // // assert_eq!(response.status(), reqwest::StatusCode::OK);

    //properties
    // let request = instance.properties("test.txt.txt", &now).unwrap();
    // let (p, _) = request.into_parts();
    // let client = reqwest::blocking::Client::new();
    // let response = client
    //     .head(&p.uri.to_string())
    //     .headers(p.headers)
    //     .send()
    //     .unwrap();
    // use std::convert::TryFrom;
    // let h = convert_response(response).unwrap();
    // let res = crate::blob::PropertiesResponse::try_from(h).unwrap();
    // println!("res:{}", res.last_modified);
    // assert_eq!("aa", res.last_modified);

    //list
    let request = instance.list("test.txt.txt", &now).unwrap();
    let (p, _) = request.into_parts();
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&p.uri.to_string())
        .headers(p.headers)
        .send()
        .unwrap();
    let a = blob::Blob::parse_list_body(&(response.text().unwrap()).trim_start_matches('\u{feff}'));
    // println!("a : {:#?}", a);
    assert_eq!(format!("a : {:#?}", a), "");
}

fn convert_response(
    res: reqwest::blocking::Response,
) -> Result<http::Response<bytes::Bytes>, Error> {
    let mut builder = http::Response::builder()
        .status(res.status())
        .version(res.version());

    let headers = builder
        .headers_mut()
        .ok_or_else(|| anyhow!("failed to convert response headers"))?;

    headers.extend(
        res.headers()
            .into_iter()
            .map(|(k, v)| (k.clone(), v.clone())),
    );

    let body = res.bytes()?;

    Ok(builder.body(body)?)
}
