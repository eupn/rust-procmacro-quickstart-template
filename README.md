# rust-procmacro-quickstart-template

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
cargo run
```
