use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use rust_actix_diesel_auth_scaffold::errors::AuthError;
use rust_actix_diesel_auth_scaffold::DbPool;

use crate::models::review::{NewReview, NewReviewJ, Review};
use crate::schema::reviews::dsl::reviews;
use crate::schema::reviews::{reviewee, username};

#[derive(serde::Deserialize, serde::Serialize)]
struct Reviews {
    reviews: Vec<Review>,
}

#[actix_web::get("/review")]
pub async fn read(
    pool: actix_web::web::Data<DbPool>,
    credentials: actix_web_httpauth::extractors::bearer::BearerAuth,
) -> Result<actix_web::web::Json<Reviews>, AuthError> {
    let mut conn = pool.get()?;

    // 0. check token username vs username in request
    if let Some((username_s, _)) = credentials.token().split_once(":") {
        use diesel::ExpressionMethods;
        let reviews_vec = reviews
            .filter(username.eq(username_s))
            .load::<Review>(&mut conn)?;

        return Ok(actix_web::web::Json(Reviews {
            reviews: reviews_vec,
        }));
    }
    Err(AuthError::NotFound("User does not have associated review"))
}

#[actix_web::post("/review")]
pub async fn upsert(
    pool: actix_web::web::Data<DbPool>,
    form: actix_web::web::Json<NewReviewJ>,
    credentials: actix_web_httpauth::extractors::bearer::BearerAuth,
) -> Result<actix_web::web::Json<Review>, AuthError> {
    let mut conn = pool.get()?;

    // 0. check token username vs username in request
    if let Some((username_s, _)) = credentials.token().split_once(":") {
        // 1. upsert review
        let mut inner = form.into_inner();
        inner.username = Some(username_s.to_string());
        let new_review_vals = NewReview::from(&inner);
        let review = diesel::insert_into(reviews)
            .values(&new_review_vals)
            .on_conflict((reviewee, username))
            .do_update()
            .set(&new_review_vals)
            .returning(Review::as_returning())
            .get_result(&mut conn)?;
        return Ok(actix_web::web::Json(review));
    }

    Err(AuthError::HttpError(500))
}
