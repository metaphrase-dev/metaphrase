table! {
    translations {
        id -> Integer,
        key -> Text,
        locale -> Text,
        content -> Nullable<Text>,
        created_at -> Text,
        deleted_at -> Nullable<Text>,
    }
}

table! {
    users {
        id -> Integer,
        email -> Text,
        hashed_password -> Text,
        created_at -> Text,
    }
}
