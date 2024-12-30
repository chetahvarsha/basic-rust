
use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
    
    map.insert("key2", 0);
    
    if map.contains_key("key1") {
        println!("Key 'key1' exists");
    } else {
        println!("Key 'key1' does not exist");
    }
    match map.get("key1") {
        Some(&value) => println!("The value for 'key1' is {}", value),
        None => println!("Key not found"),
    }


}
