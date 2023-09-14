// @generated automatically by Diesel CLI.

diesel::table! {
    batteries (id) {
        id -> Integer,
        label -> Text,
        command -> Nullable<Text>,
    }
}

diesel::table! {
    batteries_to_categories (id) {
        id -> Integer,
        battery_id -> Integer,
        category_id -> Integer,
    }
}

diesel::table! {
    categories (id) {
        id -> Integer,
        label -> Text,
        prompt -> Text,
        category_type -> Integer,
        disabled_bool -> Integer,
        extra_info -> Nullable<Text>,
    }
}

diesel::table! {
    category_option (id) {
        id -> Integer,
        category_id -> Integer,
        option_id -> Integer,
    }
}

diesel::table! {
    category_types (id) {
        id -> Integer,
        label -> Text,
    }
}

diesel::table! {
    entries (id) {
        id -> Integer,
        timestamp -> Timestamp,
        category -> Nullable<Integer>,
        value -> Nullable<Integer>,
        details -> Nullable<Text>,
    }
}

diesel::table! {
    options (id) {
        id -> Integer,
        label -> Text,
        shortcut -> Text,
        disabled_bool -> Integer,
    }
}

diesel::joinable!(batteries_to_categories -> batteries (battery_id));
diesel::joinable!(batteries_to_categories -> categories (category_id));
diesel::joinable!(categories -> category_types (category_type));
diesel::joinable!(category_option -> categories (category_id));
diesel::joinable!(category_option -> options (option_id));
diesel::joinable!(entries -> categories (category));
diesel::joinable!(entries -> options (value));

diesel::allow_tables_to_appear_in_same_query!(
    batteries,
    batteries_to_categories,
    categories,
    category_option,
    category_types,
    entries,
    options,
);
