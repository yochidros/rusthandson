use diesel::prelude::*;
use std::result::Result;

use crate::error::StringError;
use crate::item::*;
use crate::schema::items;

pub struct AccountBook {
    pub conn: SqliteConnection,
}

impl AccountBook {
    pub fn new(conn: SqliteConnection) -> AccountBook {
        AccountBook { conn }
    }

    pub fn add_item(
        &mut self,
        category: &str,
        price: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = &mut self.conn;
        let new_item = NewItem {
            category: category.to_string(),
            price,
        };
        let res = diesel::insert_into(items::table)
            .values(new_item)
            .execute(conn);
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(StringError::new(
                &format!("Error: {}", e).to_string(),
            ))),
        }
    }

    pub fn fetch_items(&mut self, limit: i32) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
        use crate::schema::items::dsl::*;
        let conn = &mut self.conn;
        let res = items
            .limit(limit.into())
            .load::<Item>(conn)
            .expect("Error loading items");
        Ok(res)
    }
}
