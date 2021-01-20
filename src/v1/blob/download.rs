use anyhow::Error;
impl super::Blob {
    pub fn download() -> Result<http::Request<std::io::Empty>, Error> {
        let req_builder = http::Request::builder();
        Ok(req_builder
            .method("GET")
            .uri("https://www.google.com")
            .body(std::io::empty())?)
    }
}
