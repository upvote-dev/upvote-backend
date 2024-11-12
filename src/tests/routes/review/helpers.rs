use crate::tests::routes::INIT_DB;

pub(crate) const USERNAMES: [&'static str; 2] = ["username2", "username3"];
pub(crate) const PASSWORD: &'static str = "password";

#[macro_export]
macro_rules! get_review_app {
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
                        .service(crate::routes::review::upsert)
                        .service(crate::routes::review::read)
                        .service(crate::routes::review::read_many),
                ),
        )
    };
}

pub(crate) async fn prepare_review_test(username: &str, password: &str) -> String {
    rust_actix_diesel_auth_scaffold::establish_connection().unwrap();
    INIT_DB.call_once(|| {
        rust_actix_diesel_auth_scaffold::db_init();
        crate::db_init();
    });

    let token =
        rust_actix_diesel_auth_scaffold::get_token(String::from(username), String::from(password))
            .await;
    token
}

pub(crate) mod test_review_api {
    pub(crate) fn post(
        token: &str,
        message: &'static str,
        username: &'static str,
    ) -> actix_http::Request {
        actix_web::test::TestRequest::post()
            .uri("/api/v0/review")
            .append_header(("Authorization", format!("Bearer {}", token)))
            .set_json(crate::models::review::NewReviewJ {
                username: Some(String::from(username)),
                message: Some(String::from(message)),
                ..crate::models::review::NewReviewJ::default()
            })
            .to_request()
    }

    pub(crate) fn get(token: &str, id: i32) -> actix_http::Request {
        actix_web::test::TestRequest::get()
            .uri(&format!("/api/v0/review/{id}"))
            .append_header(("Authorization", format!("Bearer {}", token)))
            .to_request()
    }
}
