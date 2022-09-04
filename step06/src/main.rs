mod accountbook;
mod error;
use std::io::Write;

use accountbook::*;

fn main() {
    loop {
        let n: usize = input_string("n >").parse().unwrap();
        match n {
            1 => {
                let item = input_item();
                let book = AccountBook::new("accountbook.txt".to_string());
                book.add_item(item).unwrap();
            }
            2 => {
                let book = AccountBook::new("accountbook.txt".to_string());
                let items = book.fetch_items(10).unwrap();
                show_items(&items);
            }
            _ => break,
        }
    }
}

fn input_item() -> Item {
    let category: String = input_string("category >");
    let price: i32 = input_string("price > ").parse().unwrap();
    Item { category, price }
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
        println!("{}: {}円", item.category, item.price);
    }
    println!("===========================");
}
