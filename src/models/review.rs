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

impl Default for Review {
    fn default() -> Self {
        Self {
            id: 0,
            username: String::new(),
            reviewee: String::new(),
            reviewee_kind: String::from("product"),
            vote: 0,
            message: None,
            photo_url: None,
            video_url: None,
            created_at: std::time::SystemTime::now(),
        }
    }
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

/// Create a new Review with this record
#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct NewReviewJ {
    /// Optional username (regardless of whether set uses username from access token)
    #[schema(example = rust_actix_diesel_auth_scaffold::option_default::<String>)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// Unique identifier to object being reviewed
    #[schema(example = "item0_barcode")]
    pub reviewee: String,

    /// Type of object being reviewed
    #[schema(example = "product")]
    pub reviewee_kind: String,

    /// Appraisal (e.g., `-1` is downvote, `1` is upvote)
    #[schema(example = 1i16)]
    pub vote: i16,

    /// Optional free-text review
    #[schema(example = rust_actix_diesel_auth_scaffold::option_default::<String>)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Optional image URL to photo of reviewee
    #[schema(example = rust_actix_diesel_auth_scaffold::option_default::<String>)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,

    /// Optional video URL to video of reviewee
    #[schema(example = rust_actix_diesel_auth_scaffold::option_default::<String>)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_url: Option<String>,
}

impl Default for NewReviewJ {
    fn default() -> Self {
        Self {
            username: None,
            reviewee: String::new(),
            reviewee_kind: String::from("product"),
            vote: 0,
            message: None,
            photo_url: None,
            video_url: None,
        }
    }
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
