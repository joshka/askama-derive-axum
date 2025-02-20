//! Derive macro for implementing `IntoResponse` for Askama templates.
//!
//! This crate provides a derive macro for implementing `IntoResponse` for Askama templates. This
//! allows you to use Askama templates as responses in Axum applications. It is a replacement for
//! the `askama_axum` crate, which will be no longer available in askama 0.13. See [askama#1128] and
//! [askama#1119] for more information.
//!
//! [askama#1128]: https://github.com/rinja-rs/askama/issues/1128
//! [askama#1119]: https://github.com/rinja-rs/askama/issues/1119
//!
//! # Example
//!
//! ```rust
//! use askama::Template;
//! use askama_derive_axum::IntoResponse;
//! use axum_core::response::IntoResponse;
//!
//! #[derive(Template, IntoResponse)]
//! #[template(source = "Hello {{name}}", ext = "html")]
//! struct TestTemplate {
//!     name: String,
//! }
//!
//! async fn index() -> TestTemplate {
//!     TestTemplate {
//!         name: "world!".to_string(),
//!     }
//! }
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(IntoResponse)]
pub fn into_response_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_into_response(&ast)
}

fn impl_into_response(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ::axum_core::response::IntoResponse for #name {
            fn into_response(self) -> ::axum_core::response::Response {
                use ::askama::Template;
                use ::axum_core::{
                    body::Body,
                    response::{IntoResponse, Response},
                };
                use ::http::{header::{CONTENT_TYPE, HeaderValue}, StatusCode};

                match self.render() {
                    Ok(body) => {
                        let headers = [(CONTENT_TYPE, HeaderValue::from_static("text/html"))];
                        (headers, body).into_response()
                    }
                    Err(e) => {
                        #[cfg(feature = "tracing")]
                        ::tracing::error!("Failed to render template: {e}");
                        StatusCode::INTERNAL_SERVER_ERROR.into_response()
                    }
                }
            }
        }
    };
    gen.into()
}
