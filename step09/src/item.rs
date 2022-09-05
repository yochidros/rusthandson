use crate::schema::items;
use diesel::prelude::*;
#[derive(Queryable)]
pub struct Item {
    pub id: Option<i32>,
    pub category: String,
    pub price: i32,
}
#[derive(Insertable)]
#[diesel(table_name = items)]
pub struct NewItem {
    pub category: String,
    pub price: i32,
}
