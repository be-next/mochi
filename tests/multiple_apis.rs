mod common;

use axum::body::Body;
use axum::extract::Request;
use axum::http::StatusCode;

use crate::common::{setup_service, string_body};
use tower::ServiceExt;

#[tokio::test]
async fn multiple_apis() {
    let app = setup_service("./tests/multiple_apis");

    let response = app()
        .oneshot(
            Request::post("/system/mvp/route1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(string_body(response).await, "content");

    let response = app()
        .oneshot(
            Request::patch("/system/mvp/route2")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::from_u16(303).unwrap());
    assert_eq!(string_body(response).await, "{\n  \"test\": \"2\"\n}");

    let response = app()
        .oneshot(
            Request::get("/system/mvp2/route1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(string_body(response).await, "content");

    let response = app()
        .oneshot(
            Request::delete("/system/mvp2/route2")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::from_u16(303).unwrap());
    assert_eq!(string_body(response).await, "{\n  \"test\": \"2\"\n}");
}