use anyhow::{anyhow, Error};
use bloblock::blob;
use std::convert::TryFrom;
use time::OffsetDateTime;

#[test]
#[ignore]
fn test_with_io() {
    let account = "devstoreaccount1";
    let key =
        "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==";
    let container = "ccon";

    let file_name = "test_bloblock.txt";
    let content = bytes::Bytes::from("hello world");
    let now = OffsetDateTime::now_utc()
        .format(time::macros::format_description!(
            "[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second] GMT"
        ))
        .unwrap();

    let instance = blob::Blob::new(account, key, container, true);

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
    let last_modified = OffsetDateTime::parse(
        &res.last_modified,
        &time::format_description::well_known::Rfc2822,
    );
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

    let resp = blob::parse_list_body((response.text().unwrap()).trim_start_matches('\u{feff}'));

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
