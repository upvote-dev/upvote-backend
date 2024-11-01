// @generated automatically by Diesel CLI.

diesel::table! {
    clients (id) {
        id -> Int4,
        client_id -> Varchar,
        client_secret -> Varchar,
        redirect_uri -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    profiles (alias) {
        #[max_length = 25]
        alias -> Varchar,
        #[max_length = 50]
        username -> Varchar,
        rank -> Text,
        coins -> Int4,
        #[max_length = 2048]
        profile_image_url -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    reviews (id) {
        id -> Int4,
        reviewee -> Text,
        reviewee_kind -> Text,
        #[max_length = 50]
        username -> Varchar,
        vote -> Int2,
        message -> Nullable<Text>,
        #[max_length = 2048]
        photo_url -> Nullable<Varchar>,
        #[max_length = 2048]
        video_url -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (username) {
        #[max_length = 50]
        username -> Varchar,
        password_hash -> Text,
        role -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(profiles -> users (username));
diesel::joinable!(reviews -> users (username));

diesel::allow_tables_to_appear_in_same_query!(clients, profiles, reviews, users,);
