#[derive(
    diesel::prelude::Queryable,
    diesel::Selectable,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Debug,
)]
#[diesel(table_name = crate::schema::reviews)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(rust_actix_diesel_auth_scaffold::models::user::User, foreign_key = username))]
pub struct Review {
    pub id: i32,
    pub reviewee: String,
    pub reviewee_kind: String,
    pub username: String,
    pub vote: i16,
    pub message: Option<String>,
    pub photo_url: Option<String>,
    pub video_url: Option<String>,
    pub created_at: std::time::SystemTime,
}

#[derive(diesel::prelude::Insertable, diesel::AsChangeset, Debug)]
#[diesel(table_name = crate::schema::reviews)]
pub struct NewReview<'a> {
    pub username: &'a str,
    pub reviewee: &'a str,
    pub reviewee_kind: &'a str,
    pub vote: i16,
    pub message: Option<&'a str>,
    pub photo_url: Option<&'a str>,
    pub video_url: Option<&'a str>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct NewReviewJ {
    pub username: Option<String>,
    pub reviewee: String,
    pub reviewee_kind: String,
    pub vote: i16,
    pub message: Option<String>,
    pub photo_url: Option<String>,
    pub video_url: Option<String>,
}

impl<'a> From<&'a NewReviewJ> for NewReview<'a> {
    fn from(value: &'a NewReviewJ) -> Self {
        Self {
            username: value.username.clone().unwrap().leak(), // this is guaranteed to succeed
            reviewee: &value.reviewee,
            reviewee_kind: &value.reviewee_kind,
            vote: value.vote,
            message: match &value.message {
                Some(s) => Some(s.as_str()),
                None => None,
            },
            photo_url: match &value.photo_url {
                Some(s) => Some(s.as_str()),
                None => None,
            },
            video_url: match &value.video_url {
                Some(s) => Some(s.as_str()),
                None => None,
            },
        }
    }
}
