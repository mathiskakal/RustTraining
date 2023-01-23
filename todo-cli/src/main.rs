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
        self.map.insert(key, true);
    }
    // simple method to store on disk
    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }
}
