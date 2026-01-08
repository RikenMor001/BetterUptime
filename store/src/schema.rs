pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "website_status"))]
    pub struct WebsiteStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::schema::sql_types::WebsiteStatus;

    website (id) {
        id -> Text,
        url -> Text,
        time_added -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    region (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::schema::sql_types::WebsiteStatus;

    website_tick (id) {
        id -> Text,
        website_id -> Text,
        region_id -> Text,
        response_time_ms -> Int4,
        status -> WebsiteStatus,
    }
}

diesel::joinable!(website_tick -> region (region_id));
diesel::joinable!(website_tick -> website (website_id));

diesel::allow_tables_to_appear_in_same_query!(
    website,
    region,
    website_tick,
);
