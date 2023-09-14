// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Integer,
        label -> Text,
        prompt -> Text,
        category_type -> Integer,
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
    entries (id) {
        id -> Integer,
        timestamp -> Timestamp,
        entry_number -> Integer,
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
    }
}

diesel::allow_tables_to_appear_in_same_query!(categories, category_option, entries, options,);
