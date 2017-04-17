# knock

[![Build Status](https://travis-ci.org/nsheremet/knock.svg?branch=master)](https://travis-ci.org/nsheremet/knock)
[![Build status](https://ci.appveyor.com/api/projects/status/vmxad9a9124fnjtm?svg=true)](https://ci.appveyor.com/project/nsheremet/knock)
[![Crates.io](https://img.shields.io/crates/v/knock.svg)](https://crates.io/crates/knock)

Knock is a simple HTTP Client for Rust

[Documentation](https://docs.rs/crate/knock/)

## Installation

```toml
# Cargo.toml

[dependencies]
knock = "0.1"
```

## Usage

An example client looks like:

```rust
extern crate knock;

use knock::HTTP;

fn main() {
    let http = HTTP::new("https://google.com").unwrap();
    let response = http.get().send();
}
```

For sending POST requests with custom headers

```rust
extern crate knock;

use knock::HTTP;
use std::collections::HashMap;

fn main() {
    let http = HTTP::new("https://google.com").unwrap();
    let mut body = HashMap::new();
    let mut headers = HashMap::new();

    body.insert("file", Data::File("/path/to/file.file"));
    body.insert("field", Data::String("value"));

    headers.insert("Content-Type", "multipart/form-data");

    let response = http.post().body(body).header(headers).send();
}
```

# License

`knock` is primarily distributed under the terms of Mozilla Public License 2.0.

See LICENSE for details.
