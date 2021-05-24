# bloblock

A [sans I/O](https://sans-io.readthedocs.io/) libray for [Azure Blob Storage](https://azure.microsoft.com/en-gb/services/storage/blobs/).

[![Crates.io](https://img.shields.io/crates/v/bloblock.svg)](https://crates.io/crates/bloblock)
[![Docs](https://docs.rs/bloblock/badge.svg)](https://docs.rs/bloblock)
[![dependency status](https://deps.rs/repo/github/m0ssc0de/bloblock/status.svg)](https://deps.rs/repo/github/m0ssc0de/bloblock)
[![Build Status](https://github.com/m0ssc0de/bloblock/workflows/CI/badge.svg)](https://github.com/m0ssc0de/bloblock/actions?workflow=CI)

## Why?

* You want to control how you actually make HTTP requests against Azure Blob Storage.
* You want to have more control over your dependencies, and not be bottlenecked for sticking to a particular version, or quickly upgrading, your HTTP related crates.

## Why not?

* This crate only supports some operations.
* There are several other Azure crates available that have many more features and are easier to work with, if you don't care about what HTTP clients they use.
* This crate requires more boilerplate to work with.

## Example

```rust
use bloblock::blob;
use chrono::{DateTime, Utc};

let account = "{an azure account name}";
let key = "{an zure account master key}";
let container = "{a container name}";
let instance = blob::Blob::new(&account, &key, &container, false);

let file_name = "test_bloblock.txt";
let now = Utc::now().format("%a, %d %b %Y %T GMT").to_string();
let request = instance.download(file_name, &now).unwrap();

let (p, _) = request.into_parts();

// you can alternate `reqwest` whit another one
let client = reqwest::blocking::Client::new();
let response = client
    .get(&p.uri.to_string())
    .headers(p.headers)
    .send()
    .unwrap();
```

## Contributing

I gladly accept any issues on any topic.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## TODO

- [✓] complete readme.
- [✓] add azurite into action pipeline.
- [ ] refactor for elegant.
- [ ] follow `https://rust-lang.github.io/api-guidelines/`