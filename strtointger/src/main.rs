fn my_atoi(s: &str) -> i32 {
    const INT_MIN: i32 = -(2_i64.pow(31)) as i32;
    const INT_MAX: i32 = (2_i64.pow(31) - 1) as i32;

    let mut current_position = 0;
    let mut is_negative = false;
    let mut result = 0;
    let bytes = s.as_bytes();

    // Skip leading whitespaces
    while current_position < bytes.len() && bytes[current_position].is_ascii_whitespace() {
        current_position += 1;
    }

    // If string is empty after trimming
    if current_position == bytes.len() {
        return 0;
    }

    // Check for optional sign
    if bytes[current_position] == b'+' {
        current_position += 1;
    } else if bytes[current_position] == b'-' {
        is_negative = true;
        current_position += 1;
    }

    // Convert number
    while current_position < bytes.len() && bytes[current_position].is_ascii_digit() {
        let digit = (bytes[current_position] - b'0') as i32;

        // Check for overflow and underflow
        if result > (INT_MAX - digit) / 10 {
            return if is_negative { INT_MIN } else { INT_MAX };
        }

        result = result * 10 + digit;
        current_position += 1;
    }

    // Apply sign
    if is_negative {
        -result
    } else {
        result
    }
}

fn main() {
    let input_string = "the string to integer " ;
    for c in input_string.chars() {
        println!("The ASCII value of '{}' is {}", c, c as u8);
    }
    println!("{}", my_atoi("42"));             // Output: 42
    println!("{}", my_atoi("   -42"));         // Output: -42
    println!("{}", my_atoi("4193 with words"));// Output: 4193
    println!("{}", my_atoi(" num of 8976 ")); // Output: 0
    println!("{}", my_atoi("-91283472332"));  // Output: -2147483648
}