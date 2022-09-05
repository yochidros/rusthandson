use diesel::prelude::*;
use std::result::Result;

use crate::error::StringError;
use crate::item::*;
use crate::schema::items;

pub struct AccountBook {
    pub conn: SqliteConnection,
}
pub struct Summary {
    pub category: String,
    pub count: i64,
    pub sum: i64,
}

impl Summary {
    pub fn avg(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            (self.sum / self.count) as f64
        }
    }
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

    pub fn get_summaries(&mut self) -> Result<Vec<Summary>, Box<dyn std::error::Error>> {
        use crate::schema::items::dsl::*;
        use diesel::dsl::*;
        use diesel::prelude::*;
        let conn = &mut self.conn;
        let res: Vec<(String, i64, Option<i64>)> = items
            .group_by(category)
            .select((category, count(category), sum(price)))
            .load::<(String, i64, Option<i64>)>(conn)
            .expect("ERROR aggreagte summaries");
        let summaries: Vec<Summary> = res
            .iter()
            .flat_map(|r| match r.2 {
                Some(sum) => Some(Summary {
                    category: r.0.clone(),
                    count: r.1,
                    sum,
                }),
                None => None,
            })
            .collect();
        Ok(summaries)
    }
}
