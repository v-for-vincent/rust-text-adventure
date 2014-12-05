struct Item(&'static str);

struct Location {
    description: &'static str,
    items: Vec<Item>,
}

// how to do actions?
// as an enum? sounds reasonable.
// what properties do they have?
// a keyword which is typed in
// one or more arguments, possibly a variable number
// something which happens when they are invoked

// "look": print the area description
// "take + item": move the item to the player's inventory
// "go + direction": move to the area in that direction
// "use + item (+ object)": maybe remove the item, invoke a (targeted?) effect

// one option: just match the enum value, parse the rest and act based on a match
enum Action {
    Look,
    Take,
    Go,
    Use,
}

fn main() {
    let item1 = Item("Little key");
    let item2 = Item("Big key");
    let hall = Location{ description: "A wide open space. Streams of lava flow on both sides of the entry gate.",
                         items: vec![item1, item2] };
}
