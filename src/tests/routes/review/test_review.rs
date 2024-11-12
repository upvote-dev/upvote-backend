use actix_http::body::MessageBody;

use crate::get_review_app;
use crate::models::review::Review;
use crate::tests::routes::review::helpers::{
    prepare_review_test, test_review_api, PASSWORD, USERNAMES,
};

const MESSAGE: &'static str = "my message here";

#[actix_web::test]
async fn test_upsert_review() {
    const USERNAME: &'static str = USERNAMES[0];
    let app = get_review_app!().await;
    let token = prepare_review_test(USERNAME, PASSWORD).await;
    let req = test_review_api::post(&token, MESSAGE, USERNAME);
    let resp = actix_web::test::call_service(&app, req).await;
    let status = resp.status();
    let bytes = resp.into_body().try_into_bytes().unwrap();
    assert_eq!(status, actix_web::http::StatusCode::OK);
    /* let resp_body_as_str = std::str::from_utf8(&resp_body_as_bytes).unwrap();
    println!("resp_body_as_str = {:#?}", resp_body_as_str); */
    let review: Review = serde_json::from_slice(&bytes).unwrap();
    let expect = Review {
        id: review.id,
        username: String::from(USERNAME),
        created_at: review.created_at,
        message: Some(String::from(MESSAGE)),
        ..Review::default()
    };
    assert_eq!(review, expect);
}

#[actix_web::test]
async fn test_read_review() {
    const USERNAME: &'static str = USERNAMES[1];
    let app = get_review_app!().await;
    let token = prepare_review_test(USERNAME, PASSWORD).await;
    let req = test_review_api::post(&token, MESSAGE, USERNAME);
    let upserted_review: Review = actix_web::test::call_and_read_body_json(&app, req).await;

    let req = test_review_api::get(&token, upserted_review.id);
    let read_review: Review = actix_web::test::call_and_read_body_json(&app, req).await;
    let expect = Review {
        id: read_review.id,
        username: String::from(USERNAME),
        created_at: read_review.created_at,
        message: Some(String::from(MESSAGE)),
        ..Review::default()
    };
    assert_eq!(upserted_review, read_review);
    assert_eq!(read_review, expect);
}
