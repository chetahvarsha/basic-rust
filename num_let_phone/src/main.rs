struct Solution{}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {                                              
        if digits.is_empty() {
            return vec![];
        }
        let phone = vec![
            "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
        ];
        let mut result = vec![];
        Self::helper(&digits, &phone, 0, &mut vec![], &mut result);
        result
    }

    fn helper(
        digits: &str,
        phone: &Vec<&str>,
        index: usize,
        current: &mut Vec<char>,                                            
        result: &mut Vec<String>,
    ) {
        if index == digits.len() {
            result.push(current.iter().collect());
            return;
        }
        let digit = digits.chars().nth(index).unwrap() as usize - '0' as usize;
        for c in phone[digit].chars() {
            current.push(c);
            Self::helper(digits, phone, index + 1, current, result); 
            current.pop();
        }
    }
}

fn main() {
    let digits = "87".to_string();
    let result = Solution::letter_combinations(digits);
    println!("{:?}", result);
}