struct Item(str);

struct Location {
    // literal string as this is static
    description: Box<str>,
    items: Vec<Box<Item>>,
}

fn main() {
    println!("Hello, world!");
}
