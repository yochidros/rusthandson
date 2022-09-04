mod accountbook;
mod error;
mod item;
mod schema;
use std::io::Write;

use accountbook::*;
use diesel::{Connection, SqliteConnection};
use item::Item;

fn main() {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    loop {
        let n: usize = input_string("n >").parse().unwrap();
        match n {
            1 => {
                let item = input_item();
                let mut book = AccountBook::new(make_db_conn(&database_url));
                book.add_item(item.0.as_str(), item.1).unwrap();
            }
            2 => {
                let mut book = AccountBook::new(make_db_conn(&database_url));
                let items = book.fetch_items(10).unwrap();
                show_items(&items);
            }
            3 => {
                let mut book = AccountBook::new(make_db_conn(&database_url));
                let summaries = book.get_summaries().unwrap();
                show_summries(&summaries);
            }
            _ => break,
        }
    }
}

fn make_db_conn(database_url: &str) -> SqliteConnection {
    SqliteConnection::establish(database_url)
        .expect(&format!("Error connection to {}", database_url))
}

fn input_item() -> (String, i32) {
    let category: String = input_string("category >");
    let price: i32 = input_string("price > ").parse().unwrap();
    (category, price)
}

fn input_string(help_text: &'static str) -> String {
    let mut input = String::new();
    print!("{} ", help_text);
    let _ = std::io::stdout().flush();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim_end().to_string()
}

fn show_items(items: &Vec<Item>) {
    println!("===========================");
    for item in items.iter() {
        println!(
            "[{:04}] {}: {}円",
            item.id.unwrap_or(0),
            item.category,
            item.price
        );
    }
    println!("===========================");
}
fn show_summries(summaries: &Vec<Summary>) {
    println!("===========================");
    println!("category\tcount\tsum\tavg");
    for item in summaries.iter() {
        println!(
            "{}\t{}\t{}円\t{:.2}円",
            item.category,
            item.count,
            item.sum,
            item.avg()
        );
    }
    println!("===========================");
}
