use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Debug)]
struct Item {
    category: String,
    price: i32,
}

fn main() {
    let path = Path::new("accountbook.txt");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    let n: usize = input_string("n >").parse().unwrap();

    (0..n).for_each(|_| {
        let item = input_item();
        save_item(&mut file, item).unwrap();
    });

    match show_items() {
        Ok(_) => return,
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn input_item() -> Item {
    let category: String = input_string("category >");
    let price: i32 = input_string("price > ").parse().unwrap();
    Item { category, price }
}

fn save_item(file: &mut File, item: Item) -> Result<(), std::io::Error> {
    let line = format!("{} {}\n", item.category, item.price);
    file.write(line.as_bytes())?;
    return Ok(());
}

fn input_string(help_text: &'static str) -> String {
    let mut input = String::new();
    print!("{} ", help_text);
    let _ = io::stdout().flush();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim_end().to_string()
}

#[derive(Debug)]
struct StringError {
    msg: String,
}
impl StringError {
    fn new(msg: &str) -> StringError {
        StringError {
            msg: msg.to_string(),
        }
    }
}

impl std::fmt::Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for StringError {
    fn description(&self) -> &str {
        &self.msg
    }
}

fn show_items() -> Result<(), Box<dyn Error>> {
    let path = Path::new("accountbook.txt");
    let mut file = File::open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("===========================");
    for line in contents.lines() {
        let content: Vec<&str> = line.split(" ").collect();
        if content.len() != 2 {
            return Err(StringError {
                msg: format!("split error {}", content.len()),
            }
            .into());
        }
        let category = content[0];
        let price: i32 = content[1].parse().expect("parse error");
        println!("{}: {}円", content[0], content[1]);
    }
    println!("===========================");
    Ok(())
}
