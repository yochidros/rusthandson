#[derive(Debug)]
struct Item {
    category: String,
    price: i32,
}

fn main() {
    let n: usize = input_string("n >").parse().unwrap();
    let mut items: Vec<Item> = Vec::with_capacity(n);

    (0..n).for_each(|_| {
        let item = input_item();
        items.push(item);
    });

    show_items(&items)
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

fn show_items(items: &[Item]) {
    println!("===========================");
    items.iter().for_each(|item| {
        println!("{}: {}円", item.category, item.price);
    });
    println!("===========================");
}
