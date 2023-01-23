// Imports
use std::str::FromStr;
// save some time typing
use std::{collections::HashMap, io::Read};



// Definitions
struct Todo {
    // use rust built in hashmap to store k/v pairs
    map: HashMap<String, bool>,
}

impl Todo {

    // 3/3 : BONUS: updated new() function to handle JSON instead of txt IO
    fn new() -> Result<Todo, std::io::Error> {
        //open db.json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;
        // serialize JSON as HashMap
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            // Err(e) if e.is_eof() is a match guard that allows us to detect if serde returns a premature EOF error
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occured: {}", e),
        }
    }

    // 1/3 : function to read content of the file and give back Todo populated with the previously stored values
    // note that this is not a method since it does not take self as an argument.
    /*
    fn new() -> Result<Todo, std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            // will create if not already present
            .create(true)
            .read(true)
            .open("db.txt")?;

        
        // Create a new empty string and assigns it to a variable named content
        let mut content = String::new();

        // read to output in content
        f.read_to_string(&mut content)?;

        // creates a map from content and then
        let map: HashMap<String, bool> = content
            // converts string content into iterator of its lines
            .lines()
            // applies closure to each line, splitting each with a tab and collects resulting parts into a vector
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            // then we transform it into a tuple for convenience
            // applies a closure to the vector, it takes the first element as key and second as value
            .map(|v| (v[0], v[1]))
            // applies closure to each key-value tuple and converts key to a String and value to a bool
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            // collects all the key-value tuple into the map variable.
            .collect();

        // creates a new instance of Todo with the map variable and wraps it in an Ok varia,t of the result type
        Ok(Todo { map })
    }
    */

    // 2/3 : Alternative way of doing in a less functional style
    /*
    fn new() -> Result<Todo, std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;

        let HashMap::new();

        for entries in content.lines() {
            //split and bind values
            let mut values = entries.split('\t');
            let key = values.next().expect("No Key");
            let val = values.next().expect("No Value");
            // insert them in hashmap
            map.insert(String::from(key), bool::from_str(val).unwrap());
        }
        // Return Ok
        Ok(Todo { map })
    }
    */

    //simple method to insert a new item into map pass true as a value
    fn insert (&mut self, key: String) {
        self.map.insert(key, true);
    }

    // 1/2 : simple method to store on disk
    // important to notice that this time the method doesn't take ownership
    // of self, in order to prevent us from updating the map (since it would no longer be accessible)
    /*
    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }
    */
    
    // 2/2 : update of the save method 
    // This time we return a Box containing a Rust generic error implementation. A box is a pointer to an allocation in memory.
    // since we may return either a file system error when opening the file, or a serde error when converting it, we don't really 
    // know which of the two our function may return. Therefore, we return a pointer to the possible error instead of the error itself
    // so that the caller will handle them
    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        // open db.Json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;
        // write to file with serde
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    // method to mark a task as done
    fn complete(&mut self, key: &String) -> Option<()> {
        // The method will return the result of a match expression, which is either an empty Some() or None
        // self.map.get_mu will give us a mutable reference to the value of the key or None if the value is
        // not present in collection
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}

// Main function
fn main() {
    // get the arguments from the CLI
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    println!("{:?},{:?}", action, item);

    // create a mutable instance of the todo struct
    let mut todo = Todo::new().expect("Initialisation of DB failed...");

    // handle if action == add
    if action == "add" {
        todo.insert(item);
        //match the result returned from the save function and print a message on screen for both cases
        match todo.save() {
                Ok(_) => println!("ToDo saved"),
                Err(why) => println!("An error occured: {}", why),
        }
    // handle if action == complete
    } else if action == "complete"  {
        // we match the Option returned by the complete method
        // we passed item as a reference with &item to the complete method so that the value is still 
        // owned by this function. This means we can use it for our print macro on the following line. 
        match todo.complete(&item) {
            // if the case is none we print a warning for the user for a better experience
            None => println!("'{}' is not present in the list", item),
            // if we detect that Some value has returned, we call todo.save to store the change
            // permanently into our file.
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occured: {}", why),
            },
        }
    }
}