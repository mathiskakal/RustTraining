// save some time typing
use std::collections::HashMap



// Main function
fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    println!("{:?},{:?}", action, item);

}

// Definitions
struct Todo {
    // use rust buult in hashmap to store k/v pairs
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert (&mut self, key: String) {
        //insert a new item into map
        //pass true as a value
        self.map.insert(k, true);
    }
}
