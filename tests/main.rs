use askama::Template;
use askama_derive_axum::IntoResponse;
use axum_core::response::IntoResponse;
use http::StatusCode;
use http_body_util::BodyExt;

#[derive(Template, IntoResponse)]
#[template(source = "Hello {{name}}", ext = "html")]
struct TestTemplate {
    name: String,
}

#[tokio::test]
async fn test_derive_into_response() {
    let template = TestTemplate {
        name: "world!".to_string(),
    };

    let response = template.into_response();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(body, "Hello world!");
}
