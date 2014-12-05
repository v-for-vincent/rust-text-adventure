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

// alternatively, instantiate actions at runtime
// write a struct for each one with arity and synonyms
// problem: each one is associated with a function of some sort
// do not want to go adding these at runtime if it can be avoided

fn parse(command: String) {
    let tokens = command.split(' ');
    match tokens[0] {
        "look" => println!("You look around."),
        "take" => println!("You take something."),
        "go" => println!("You go somewhere."),
        "use" => println!("You use something."),
        _ => println!("Unknown command!"),
    }
    // look should just check for arity 0 invoke an associated function
    // take should parse rest of the command, see if there is an item with a name equal to the rest of the command
    // go should parse exactly one more token and go in that direction... how?
    // use should parse either one or two tokens and act accordingly
}

fn main() {
    let item1 = Item("Little key");
    let item2 = Item("Big key");
    let hall = Location{ description: "A wide open space. Streams of lava flow on both sides of the entry gate.",
                         items: vec![item1, item2] };
}
