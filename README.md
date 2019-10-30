# rust-procmacro-quickstart-template

[![travis-ci badge](https://api.travis-ci.com/eupn/rust-procmacro-quickstart-template.svg?branch=master)](https://travis-ci.com/eupn/rust-procmacro-quickstart-template)

A base `cargo generate` template for building a 
[procedural macro](https://doc.rust-lang.org/reference/procedural-macros.html#procedural-macros) project crate.

### How to use it?

1. Install [cargo-generate](https://github.com/ashleygwilliams/cargo-generate):
```bash
cargo install cargo-generate
```
2. Generate your procedural macro crate project:
```bash
cargo generate --git https://github.com/eupn/rust-procmacro-quickstart-template --name projectname
```
3. Run generated template project:
```bash
cd projectname
cargo build
```

### License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
