fn main() {
    let tuple = (42, 3.14, 'a');
    let first = tuple.0;
    let second = tuple.1;
    println!("First: {}, Second: {}", first, second);
    let coords = get_coordinates();
    println!("Coordinates: ({}, {})", coords.0, coords.1);
}

fn get_coordinates() -> (i32, i32) {
    (10, 20)
}


    

