# Askama-derive-axum

[![crate-badge]][crate]
[![docs-badge]][docs]
[![repo-badge]][repo]
[![license-badge]][license]

[crate-badge]: https://img.shields.io/crates/v/askama-derive-axum?logo=rust
[crate]: https://crates.io/crates/askama-derive-axum
[docs-badge]: https://img.shields.io/badge/docs-askama--derive--axum-e05d44?logo=rust
[docs]: https://docs.rs/askama-derive-axum
[repo-badge]: https://img.shields.io/badge/repo-joshka/askama--derive--axum-8957E5?logo=github
[repo]: https://github.com/joshka/askama-derive-axum
[license-badge]: https://img.shields.io/crates/l/askama-derive-axum
[license]: #license

<!-- cargo-rdme start -->

## Status

This crate is no longer supported. For a maintained alternative, use
[`askama_web`](https://crates.io/crates/askama_web) for Askama + Axum integration.

Derive macro for implementing [`IntoResponse`] for Askama templates.

This crate provides a derive macro for implementing [`IntoResponse`] for Askama templates. This
allows you to use Askama templates as responses in Axum applications. It is a replacement for
the `askama_axum` crate, which will be no longer available in askama 0.13. See [askama#1128] and
[askama#1119] for more information.

[askama#1128]: https://github.com/rinja-rs/askama/issues/1128
[askama#1119]: https://github.com/rinja-rs/askama/issues/1119
[`IntoResponse`]: axum_core::response::IntoResponse

## Example

```rust
use askama::Template;
use askama_derive_axum::IntoResponse;

#[derive(Template, IntoResponse)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    body: String,
}

async fn index() -> IndexTemplate {
    IndexTemplate {
        title: "My Blog".to_string(),
        body: "Hello, world!".to_string(),
    }
}
```

<!-- cargo-rdme end -->

## License

Copyright (c) Josh McKinney

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE] or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT] or <http://opensource.org/licenses/MIT>)

at your option.

[LICENSE-APACHE]: /LICENSE-APACHE
[LICENSE-MIT]: /LICENSE-MIT

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

See [CONTRIBUTING.md](/CONTRIBUTING.md).
