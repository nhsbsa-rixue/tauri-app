use super::schema::{dish_types, dishes, posts};
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(serde::Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = dishes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(serde::Serialize)]
pub struct Dishes {
    pub id: i32,
    pub name_en: String,
    pub name_cn: String,
    pub menu_type: String,
    pub price: f32,
    pub is_set_meal: bool,
    pub is_attached: bool,
    pub is_selectable: bool,
    pub notes: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = dish_types)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(serde::Serialize)]
pub struct DishTypes {
    pub id: i32,
    pub name_en: String,
    pub name_cn: String,
}
