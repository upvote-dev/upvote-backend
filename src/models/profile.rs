#[derive(
    diesel::prelude::Queryable,
    diesel::Selectable,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Debug,
)]
#[diesel(table_name = crate::schema::profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(rust_actix_diesel_auth_scaffold::models::user::User, foreign_key = username))]
pub struct Profile {
    pub alias: String,
    pub username: String,
    pub rank: String,
    pub coins: i32,
    pub profile_image_url: Option<String>,
    pub created_at: std::time::SystemTime,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            alias: String::new(),
            username: String::new(),
            rank: String::from(DEFAULT_RANK),
            coins: 0,
            profile_image_url: None,
            created_at: std::time::SystemTime::now(),
        }
    }
}

pub const DEFAULT_RANK: &'static str = "paladin";

#[derive(diesel::prelude::Insertable, diesel::AsChangeset, Debug)]
#[diesel(table_name = crate::schema::profiles)]
pub struct NewProfile<'a> {
    pub alias: &'a str,
    pub username: &'a str,
    pub rank: &'a str,
    pub coins: i32,
    pub profile_image_url: Option<&'a str>,
}

const DEFAULT_USERNAME: fn() -> String = || String::from("DEFAULT_USERNAME");

/// Create a new Profile with this record
#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct NewProfileJ {
    /// Optional alias (if set, this is publicly used instead of username)
    #[schema(example = rust_actix_diesel_auth_scaffold::option_default::<String>)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    /// Optional username (regardless of whether set uses username from access token)
    #[schema(example = DEFAULT_USERNAME)]
    // this default gets overridden anyway
    #[serde(default = "DEFAULT_USERNAME")]
    pub username: String,

    /// Optional image URL to avatar associated with this profile
    #[schema(example = rust_actix_diesel_auth_scaffold::option_default::<String>)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_image_url: Option<String>,
}

impl<'a> From<&'a NewProfileJ> for NewProfile<'a> {
    fn from(value: &'a NewProfileJ) -> Self {
        Self {
            alias: match &value.alias {
                Some(alias) => alias,
                None => &value.username,
            },
            username: &value.username,
            rank: DEFAULT_RANK,
            coins: 0,
            profile_image_url: match &value.profile_image_url {
                Some(s) => Some(s.as_str()),
                None => None,
            },
        }
    }
}
