use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::result::Result;

use crate::error::StringError;

#[derive(Debug)]
pub struct Item {
    pub category: String,
    pub price: i32,
}

#[derive(Debug)]
pub struct AccountBook {
    pub filename: String,
}

impl AccountBook {
    pub fn new(filename: String) -> AccountBook {
        AccountBook { filename }
    }

    pub fn add_item(self, item: Item) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(&self.filename);
        let file = OpenOptions::new().write(true).append(true).open(path);

        let mut file = match file {
            Ok(f) => f,
            Err(e) => {
                return Err(Box::new(StringError::new(
                    &format!("Error: {}", e).to_string(),
                )))
            }
        };

        let line = format!("{} {}\n", item.category, item.price);

        file.write(line.as_bytes())?;

        Ok(())
    }

    pub fn fetch_items(self, limit: usize) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
        let path = Path::new(&self.filename);
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                return Err(Box::new(StringError::new(
                    &format!("Error: {}", e).to_string(),
                )))
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let res = contents
            .lines()
            .rev()
            .enumerate()
            .filter(|(i, _)| i < &limit)
            .map(|(_, line)| line.split(" ").collect::<Vec<&str>>())
            .flat_map(|content| {
                if content.len() != 2 {
                    return Err(StringError {
                        msg: format!("split error {}", content.len()),
                    });
                }

                let category = content[0].to_string();
                let price: i32 = content[1].parse().expect("parse error");
                Ok(Item { category, price })
            })
            .collect();

        Ok(res)
    }
}
