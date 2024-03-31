// @generated automatically by Diesel CLI.

diesel::table! {
    days_of_week (id) {
        id -> Integer,
        #[max_length = 10]
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users_accesses (user_id, day_of_week) {
        user_id -> Integer,
        day_of_week -> Integer,
        start -> Time,
        end -> Time,
    }
}

diesel::table! {
    users_logs (id) {
        id -> Integer,
        user_id -> Nullable<Integer>,
        #[max_length = 255]
        action -> Varchar,
        timestamp -> Nullable<Timestamp>,
    }
}

diesel::joinable!(users_accesses -> days_of_week (day_of_week));
diesel::joinable!(users_accesses -> users (user_id));
diesel::joinable!(users_logs -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    days_of_week,
    users,
    users_accesses,
    users_logs,
);
