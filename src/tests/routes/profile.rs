use actix_web::body::MessageBody;

const USERNAME: &'static str = "username0";
const PASSWORD: &'static str = "password";

macro_rules! get_profile_app {
    () => {
        actix_web::test::init_service(
            actix_web::App::new()
                .app_data(actix_web::web::Data::new(
                    rust_actix_diesel_auth_scaffold::POOL.clone(),
                ))
                .service(
                    actix_web::web::scope("/api/v0")
                        .wrap(actix_web::middleware::Compat::new(
                            actix_web_httpauth::middleware::HttpAuthentication::bearer(
                                rust_actix_diesel_auth_scaffold::middleware::bearer::validator,
                            ),
                        ))
                        .service(crate::routes::profile::upsert)
                        .service(crate::routes::profile::read),
                ),
        )
    };
}

/*fn prepare_profile_app() -> actix_web::App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse<impl actix_http::body::MessageBody>,
        Error = actix_http::Error,
        InitError = (),
    >,
> */

async fn prepare_profile_test() -> String {
    rust_actix_diesel_auth_scaffold::establish_connection().unwrap();
    rust_actix_diesel_auth_scaffold::db_init();
    crate::db_init();

    let token =
        rust_actix_diesel_auth_scaffold::get_token(String::from(USERNAME), String::from(PASSWORD))
            .await;
    token
}

#[actix_web::test]
async fn test_upsert_profile() {
    let app = get_profile_app!().await;
    let token = prepare_profile_test().await;
    let req = actix_web::test::TestRequest::post()
        .uri("/api/v0/profile")
        .append_header(("Authorization", format!("Bearer {}", token)))
        .set_json(crate::models::profile::NewProfileJ {
            alias: Some(format!("{}-alias", USERNAME)),
            username: String::from(USERNAME),
            rank: None,
            coins: None,
            profile_image_url: None,
        })
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;
    let status = resp.status();
    let resp_body_as_bytes = resp.into_body().try_into_bytes().unwrap();
    // let resp_body_as_str = std::str::from_utf8(&resp_body_as_bytes).unwrap();
    assert_eq!(status, actix_web::http::StatusCode::OK);
    // println!("resp_body_as_str = {:#?}", resp_body_as_str);
    let resp_body_as_profile: crate::models::profile::Profile =
        serde_json::from_slice(&resp_body_as_bytes).unwrap();
    assert!(resp_body_as_profile.profile_image_url.is_none());
    assert_eq!(resp_body_as_profile.alias, format!("{}-alias", USERNAME));
    assert_eq!(resp_body_as_profile.username, USERNAME);
}
