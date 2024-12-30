// Define a struct named `Person`
struct Person {
    name: String,
    age: u32,
}

fn main() {
    // Create an instance of the `Person` struct
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Accessing fields of the struct
    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
} 