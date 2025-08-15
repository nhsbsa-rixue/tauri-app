diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool
    }
}

diesel::table! {
    dish_types (id) {
        id -> Integer,
        name_en -> Text,
        name_cn -> Text
    }
}

diesel::table! {
    dishes (id) {
        id -> Integer,
        name_en -> Text,
        name_cn -> Text,
        menu_type -> Text,
        price -> Float,
        is_set_meal -> Bool,
        is_attached -> Bool,
        is_selectable -> Bool,
        notes -> Nullable<Text>,
    }
}
