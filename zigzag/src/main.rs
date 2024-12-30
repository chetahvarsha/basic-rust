fn convert(s: &str, num_rows: usize) -> String {
    if num_rows == 1 || num_rows >= s.len() {
        return s.to_string();
    }

    // Initialize rows with empty strings
    let mut rows: Vec<String> = vec![String::new(); num_rows];
    let mut current_row: isize = 0;
    let mut going_down = false;

    // Process each character in the string
    for char in s.chars() {
        rows[current_row as usize].push(char);

        // Change direction at the top or bottom row
        if current_row == 0 || current_row == (num_rows - 1) as isize {
            going_down = !going_down;
        }

        // Move to the next row
        current_row += if going_down { 1 } else { -1 };
    }

    // Concatenate all rows into a single string
    rows.concat()
}

fn main() {
    let s = "i am varsha";
    let num_rows = 3;
    let result = convert(s, num_rows);
    println!("{}", result); // Output: "PAHNAPLIGSIYIR"
}

