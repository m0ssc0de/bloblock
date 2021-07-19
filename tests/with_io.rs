use anyhow::{anyhow, Error};
use bloblock::blob;
use chrono::{DateTime, Utc};
use std::convert::TryFrom;

#[test]
#[ignore]
fn test_with_io() {
    let account = "devstoreaccount1";
    let key =
        "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==";
    let container = "ccon";

    let file_name = "test_bloblock.txt";
    let content = bytes::Bytes::from("hello world");
    let now = Utc::now().format("%a, %d %b %Y %T GMT").to_string();

    let instance = blob::Blob::new(&account, &key, &container, true);

    //insert
    let request = instance.insert(file_name, content, &now).unwrap();
    let (p, b) = request.into_parts();
    let client = reqwest::blocking::Client::new();
    let response = client
        .put(&p.uri.to_string())
        .headers(p.headers)
        .body(b)
        .send()
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::CREATED);

    // download
    let request = instance.download(file_name, &now).unwrap();
    let (p, _) = request.into_parts();
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&p.uri.to_string())
        .headers(p.headers)
        .send()
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    //properties
    let request = instance.properties(file_name, &now).unwrap();

    let (p, _) = request.into_parts();
    let client = reqwest::blocking::Client::new();
    let response = client
        .head(&p.uri.to_string())
        .headers(p.headers)
        .send()
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    let hresp = convert_response(response).unwrap();
    let res = crate::blob::PropertiesResponse::try_from(hresp).unwrap();
    let last_modified = DateTime::parse_from_rfc2822(&res.last_modified);
    assert!(last_modified.is_ok());

    //list
    let request = instance.list(&now).unwrap();
    let (p, _) = request.into_parts();
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&p.uri.to_string())
        .headers(p.headers)
        .send()
        .unwrap();

    let resp = blob::parse_list_body(&(response.text().unwrap()).trim_start_matches('\u{feff}'));

    assert!(resp.is_ok());
    let the_res = resp.unwrap();
    assert!(!the_res.blobs.blob.is_empty());
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
