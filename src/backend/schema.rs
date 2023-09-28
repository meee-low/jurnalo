// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Integer,
        label -> Text,
        prompt -> Text,
        category_type -> Integer,
        disabled_bool -> Integer,
        extra_info -> Nullable<Text>,
        show_in_streaks -> Integer,
    }
}

diesel::table! {
    category_types (id) {
        id -> Integer,
        label -> Text,
    }
}

diesel::table! {
    choices (id) {
        id -> Integer,
        label -> Text,
        shortcut -> Text,
        disabled_bool -> Integer,
        category_label -> Text,
        show_in_streaks -> Integer,
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
    quizzes (id) {
        id -> Integer,
        label -> Text,
        command -> Nullable<Text>,
    }
}

diesel::table! {
    quizzes_to_categories (id) {
        id -> Integer,
        quiz_label -> Text,
        category_label -> Text,
        order -> Integer,
    }
}

diesel::joinable!(categories -> category_types (category_type));
diesel::joinable!(entries -> categories (category));
diesel::joinable!(entries -> choices (value));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    category_types,
    choices,
    entries,
    quizzes,
    quizzes_to_categories,
);
