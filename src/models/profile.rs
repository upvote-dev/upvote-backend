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

#[derive(diesel::prelude::Insertable, diesel::AsChangeset, Debug)]
#[diesel(table_name = crate::schema::profiles)]
pub struct NewProfile<'a> {
    pub alias: &'a str,
    pub username: &'a str,
    pub rank: &'a str,
    pub coins: Option<i32>,
    pub profile_image_url: Option<&'a str>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct NewProfileJ {
    pub alias: String,
    pub username: String,
    pub rank: String,
    pub coins: Option<i32>,
    pub profile_image_url: Option<String>,
}

impl<'a> From<&'a NewProfileJ> for NewProfile<'a> {
    fn from(value: &'a NewProfileJ) -> Self {
        Self {
            alias: &value.alias,
            username: &value.username,
            rank: &value.rank,
            coins: match &value.coins {
                Some(c) => Some(c.to_owned()),
                None => Some(0),
            },
            profile_image_url: match &value.profile_image_url {
                Some(s) => Some(s.as_str()),
                None => None,
            },
        }
    }
}
