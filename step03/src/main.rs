#[derive(Debug)]
struct Item {
    category: String,
    price: i32,
}

fn main() {
    println!("===========================");
    let item = input_item();
    println!("{}に{}円つかいました", item.category, item.price);
    println!("===========================");
}

fn input_item() -> Item {
    let category: String = input_string("category >");
    let price: i32 = input_string("price > ").parse().unwrap();
    Item { category, price }
}

fn input_string(help_text: &'static str) -> String {
    let mut input = String::new();
    println!("{}", help_text);
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim_end().to_string()
}
