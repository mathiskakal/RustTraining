// Imports

// save some time typing
use std::collections::HashMap;



// Definitions
struct Todo {
    // use rust built in hashmap to store k/v pairs
    map: HashMap<String, bool>,
}

impl Todo {

    //simple method to insert a new item into map pass true as a value
    fn insert (&mut self, key: String) {
        self.map.insert(key, true);
    }

    // simple method to store on disk
    // important to notice that this time the method doesn't take ownership
    // of self, in order to prevent us from updating the map (since it would no longer be accessible)
    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }
}

// Main function
fn main() {
    // get the arguments from the CLI
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    println!("{:?},{:?}", action, item);

    // create a mutable instance of the todo struct
    let mut todo = Todo {
        map: HashMap::new(),
    };

    // handle if action == add
    if action == "add" {
        todo.insert(item);
        //match the result returned from the save function and print a message on screen for both cases
        match todo.save() {
                Ok(_) => println!("ToDo saved"),
                Err(why) => println!("An error occured: {}", why),
        }
    }
}