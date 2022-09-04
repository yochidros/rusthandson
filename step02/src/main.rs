fn main() {
    let mut category = String::new();
    let mut input_price = String::new();

    println!("品目> ");
    std::io::stdin().read_line(&mut category).unwrap();

    println!("\n値段> ");
    std::io::stdin().read_line(&mut input_price).unwrap();
    let price: i32 = input_price.trim().parse().unwrap();

    println!("\n=====================");

    println!("{}に{}円使いました", category.trim_end(), price);

    println!("=====================");
}
