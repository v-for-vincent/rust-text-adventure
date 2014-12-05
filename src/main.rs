struct Item(&'static str);

struct Location {
    description: &'static str,
    items: Vec<Item>,
}

fn main() {
    let item1 = Item("Little key");
    let item2 = Item("Big key");
    let hall = Location{ description: "A wide open space. Streams of lava flow on both sides of the entry gate.",
                         items: vec![item1, item2] };
}
