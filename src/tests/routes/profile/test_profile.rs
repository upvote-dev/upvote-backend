use actix_http::body::MessageBody;

use crate::get_profile_app;
use crate::models::profile::Profile;
use crate::tests::routes::profile::helpers::{
    prepare_profile_test, test_profile_api, PASSWORD, USERNAMES,
};

#[actix_web::test]
async fn test_upsert_profile() {
    const USERNAME: &'static str = USERNAMES[0];
    let app = get_profile_app!().await;
    let token = prepare_profile_test(USERNAME, PASSWORD).await;
    let alias = format!("{}-alias", USERNAME);
    let req = test_profile_api::post(&token, &alias, USERNAME);
    let resp = actix_web::test::call_service(&app, req).await;
    let status = resp.status();
    let bytes = resp.into_body().try_into_bytes().unwrap();
    assert_eq!(status, actix_web::http::StatusCode::OK);
    /* let resp_body_as_str = std::str::from_utf8(&resp_body_as_bytes).unwrap();
    println!("resp_body_as_str = {:#?}", resp_body_as_str); */
    let profile: Profile = serde_json::from_slice(&bytes).unwrap();
    let expect = Profile {
        alias,
        username: String::from(USERNAME),
        created_at: profile.created_at,
        ..Profile::default()
    };
    assert_eq!(profile, expect);
}

#[actix_web::test]
async fn test_read_profile() {
    const USERNAME: &'static str = USERNAMES[1];
    let app = get_profile_app!().await;
    let token = prepare_profile_test(USERNAME, PASSWORD).await;
    let alias = format!("{}-alias", USERNAME);
    let req = test_profile_api::post(&token, &alias, USERNAME);
    let upserted_profile: Profile = actix_web::test::call_and_read_body_json(&app, req).await;

    let req = test_profile_api::get(&token);
    let read_profile: Profile = actix_web::test::call_and_read_body_json(&app, req).await;
    let expect = Profile {
        alias,
        username: String::from(USERNAME),
        created_at: read_profile.created_at,
        ..Profile::default()
    };
    assert_eq!(upserted_profile, read_profile);
    assert_eq!(read_profile, expect);
}
