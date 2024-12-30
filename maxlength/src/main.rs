use std::collections::HashSet;

fn longest_substring_without_repeating_chars(s: &str) -> usize {
    let mut max_length = 0;
    let mut char_set: HashSet<char> = HashSet::new();
    let mut left = 0;

    for (right, c) in s.chars().enumerate() {
        while char_set.contains(&c) {
            char_set.remove(&s.chars().nth(left).unwrap());
            left += 1;
        }
        char_set.insert(c);
        max_length = max_length.max(right as usize - left + 1);
    }

    max_length
}

fn main() {
    let s = "bbbbbb";
    println!("Longest substring without repeating characters: {}", longest_substring_without_repeating_chars(s));
}
