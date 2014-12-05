struct Item(str);

struct Location {
    // literal string as this is static
    description: str,
    items: Vec<Item>,
}

fn main() {
    println!("Hello, world!");
}
