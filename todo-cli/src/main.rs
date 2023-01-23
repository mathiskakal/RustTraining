// Imports

// save some time typing
use std::{collections::HashMap, io::Read};



// Definitions
struct Todo {
    // use rust built in hashmap to store k/v pairs
    map: HashMap<String, bool>,
}

impl Todo {

    // method to read content of the file and give back Todo populated with the previously stored values
    // note that this is not a method since it does not take self as an argument.
    fn new() -> Result<Todo, std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
        let mut content = String::new();
        // read to output in content
        f.read_to_string(&mut content)?;
        // creates a map from content and then
        let map: HashMap<String, bool> = content
            // converts string content into iterator of its lines
            .lines()
            // applies closure to each line, splitting each with a tab and collects resulting parts into a vector
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            // applies a closure to the vector, it takes the first element as key and second as value
            .map(|v| (v[0], v[1]))*
            // applies closure to each key-value tuple and converts key to a String and value to a bool
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            // collects all the key-value tuple into the map variable.
            .collect();
        // creates a new instance of Todo with the map variable and wraps it in an Ok varia,t of the result type
        Ok(Todo { map })
    }

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