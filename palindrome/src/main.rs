fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase();
    let chars: Vec<char> = s.chars().collect();
    let mut left = 0;
    let mut right = chars.len() - 1;

    while left < right {
        if chars[left] != chars[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}

fn main() {
    println!("{}", is_palindrome("radar"));  // prints: true
    println!("{}", is_palindrome("12121"));  // prints: false
}