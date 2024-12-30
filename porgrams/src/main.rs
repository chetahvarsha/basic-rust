fn main() {
    let input = "12121";
    let cleaned_input = clean_string(input);
    if is_palindrome(&cleaned_input) {
        println!("The input string is a palindrome.");
    } else {
        println!("The input string is not a palindrome.");
    }
}

// Function to clean the input string
fn clean_string(s: &str) -> String {
    s.chars()
     .filter(|c| c.is_alphanumeric())
     .map(|c| c.to_ascii_lowercase())
     .collect()
}

// Function to check if the cleaned string is a palindrome
fn is_palindrome(s: &str) -> bool {
    let reversed: String = s.chars().rev().collect();
    s == reversed
}
