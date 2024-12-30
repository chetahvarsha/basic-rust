fn main() {
    // Using an array
    let arr = [1, 2, 3, 4, 5];
    
    // Creating a slice of the entire array
    let slice: &[i32] = &arr;
    
    println!("Slice of the entire array: {:?}", slice);

    // Creating a slice of a portion of the array
    let slice_of_arr = &arr[1..4];
    println!("Slice of array from index 1 to 3: {:?}", slice_of_arr);

    // Using a vector
    let vec = vec![10, 20, 30, 40, 50];
    
    // Creating a slice of the entire vector
    let slice_of_vec: &[i32] = &vec;
    println!("Slice of the entire vector: {:?}", slice_of_vec);

    // Creating a slice of a portion of the vector
    let slice_of_vec_range = &vec[2..5];
    println!("Slice of vector from index 2 to 4: {:?}", slice_of_vec_range);
    char();
}

fn char() {
    let s = String::from("hello, world");

    // Convert to a vector of chars
    let chars: Vec<char> = s.chars().collect();
    
    // Slicing on the vector of chars
    let slice: Vec<char> = chars[0..5].to_vec();
    
    println!("Unicode-safe slice: {}", slice.iter().collect::<String>());
}
