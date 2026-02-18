// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "website_status"))]
    pub struct WebsiteStatus;
}

diesel::table! {
    notification (id) {
        id -> Text,
        user_id -> Text,
        email -> Text,
        notify_down -> Bool,
        notify_up -> Bool,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    region (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    user (id) {
        id -> Text,
        username -> Text,
        password -> Text,
    }
}

diesel::table! {
    website (id) {
        id -> Text,
        url -> Text,
        time_added -> Timestamptz,
        user_id -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::WebsiteStatus;

    website_tick (id) {
        id -> Text,
        website_id -> Text,
        region_id -> Text,
        response_time_ms -> Int4,
        status -> WebsiteStatus,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(notification -> user (user_id));
diesel::joinable!(website -> user (user_id));
diesel::joinable!(website_tick -> region (region_id));
diesel::joinable!(website_tick -> website (website_id));

diesel::allow_tables_to_appear_in_same_query!(notification, region, user, website, website_tick,);
