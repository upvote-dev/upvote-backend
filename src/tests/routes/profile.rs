use actix_web::body::MessageBody;

const USERNAMES: [&'static str; 2] = ["username0", "username1"];
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

async fn prepare_profile_test(username: &str, password: &str) -> String {
    rust_actix_diesel_auth_scaffold::establish_connection().unwrap();
    rust_actix_diesel_auth_scaffold::db_init();
    crate::db_init();

    let token =
        rust_actix_diesel_auth_scaffold::get_token(String::from(username), String::from(password))
            .await;
    token
}

#[actix_web::test]
async fn test_upsert_profile() {
    let app = get_profile_app!().await;
    let token = prepare_profile_test(USERNAMES[0], PASSWORD).await;
    let alias = format!("{}-alias", USERNAMES[0]);
    let req = actix_web::test::TestRequest::post()
        .uri("/api/v0/profile")
        .append_header(("Authorization", format!("Bearer {}", token)))
        .set_json(crate::models::profile::NewProfileJ {
            alias: Some(alias.clone()),
            username: String::from(USERNAMES[0]),
            rank: None,
            coins: None,
            profile_image_url: None,
        })
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;
    let status = resp.status();
    let bytes = resp.into_body().try_into_bytes().unwrap();
    assert_eq!(status, actix_web::http::StatusCode::OK);
    /* let resp_body_as_str = std::str::from_utf8(&resp_body_as_bytes).unwrap();
    println!("resp_body_as_str = {:#?}", resp_body_as_str); */
    let profile: crate::models::profile::Profile = serde_json::from_slice(&bytes).unwrap();
    let expect = crate::models::profile::Profile {
        alias,
        username: String::from(USERNAMES[0]),
        created_at: profile.created_at,
        ..crate::models::profile::Profile::default()
    };
    assert_eq!(profile, expect);
}

#[actix_web::test]
async fn test_read_profile() {
    let app = get_profile_app!().await;
    let token = prepare_profile_test(USERNAMES[1], PASSWORD).await;
    let alias = format!("{}-alias", USERNAMES[1]);
    let req = actix_web::test::TestRequest::post()
        .uri("/api/v0/profile")
        .append_header(("Authorization", format!("Bearer {}", token)))
        .set_json(crate::models::profile::NewProfileJ {
            alias: Some(alias.clone()),
            username: String::from(USERNAMES[1]),
            rank: None,
            coins: None,
            profile_image_url: None,
        })
        .to_request();
    let upserted_profile: crate::models::profile::Profile =
        actix_web::test::call_and_read_body_json(&app, req).await;

    let req = actix_web::test::TestRequest::get()
        .uri("/api/v0/profile")
        .append_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let read_profile: crate::models::profile::Profile =
        actix_web::test::call_and_read_body_json(&app, req).await;
    let expect = crate::models::profile::Profile {
        alias,
        username: String::from(USERNAMES[1]),
        created_at: read_profile.created_at,
        ..crate::models::profile::Profile::default()
    };
    assert_eq!(upserted_profile, read_profile);
    assert_eq!(read_profile, expect);
}
