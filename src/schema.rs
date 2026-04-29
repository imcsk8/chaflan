// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Uuid,
        name -> Text,
        venue -> Text,
        address -> Nullable<Text>,
        #[max_length = 255]
        image -> Nullable<Varchar>,
        comments -> Nullable<Text>,
        #[max_length = 255]
        contactname -> Nullable<Varchar>,
        starts_at -> Timestamp,
        ends_at -> Timestamp,
        url -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Nullable<Text>,
        email -> Text,
        password -> Nullable<Text>,
        oauth_provider -> Text,
        oauth_user_id -> Text,
        access_token -> Text,
        refresh_token -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    events,users,);
