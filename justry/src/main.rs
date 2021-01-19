use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

fn main() {
    // Create alias for HMAC-SHA256
    type HmacSha256 = Hmac<Sha256>;

    // Create HMAC-SHA256 instance which implements `Mac` trait
    let mut mac =
        HmacSha256::new_varkey(b"my secret and secure key").expect("HMAC can take key of any size");

    let StringToSign = {
        let VERB = "GET";
        let ContentEncoding = "";
        let ContentLanguage = "";
        let ContentLength = "";
        let ContentMD5 = "";
        let ContentType = "";
        let Date = "";
        let IfModifiedSince = "";
        let IfMatch = "";
        let IfNoneMatch = "";
        let IfUnmodifiedSince = "";
        let Range = "";
        let CanonicalizedHeaders =
            "x-ms-date:Fri, 26 Jun 2015 23:39:12 GMT\nx-ms-version:2015-02-21";
        let CanonicalizedResource =
            "/myaccount/mycontainer\ncomp:metadata\nrestype:container\ntimeout:20";
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            VERB,
            ContentEncoding,
            ContentLanguage,
            ContentLength,
            ContentMD5,
            ContentType,
            Date,
            IfModifiedSince,
            IfMatch,
            IfNoneMatch,
            IfUnmodifiedSince,
            Range,
            CanonicalizedHeaders,
            CanonicalizedResource,
        )
    };

    mac.update(b"GET\n\n\n\n\n\n\n\n\n\n\n\nx-ms-date:Fri, 26 Jun 2015 23:39:12 GMT\nx-ms-version:2015-02-21\n/myaccount/mycontainer\ncomp:metadata\nrestype:container\ntimeout:20");

    // `result` has type `Output` which is a thin wrapper around array of
    // bytes for providing constant time equality check
    let result = mac.finalize();
    // To get underlying array use `into_bytes` method, but be careful, since
    // incorrect use of the code value may permit timing attacks which defeat
    // the security provided by the `Output`
    let code_bytes = result.into_bytes();
    println!("Hello, world! {:x}", code_bytes);

    let mut mac =
        HmacSha256::new_varkey(b"my secret and secure key").expect("HMAC can take key of any size");
    mac.update(&StringToSign.into_bytes()[..]);
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    println!("Hello, world! {:x}", code_bytes);
}
