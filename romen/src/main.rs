fn roman_to_int(s: &str) -> i32 {
    let mut result = 0;
    let mut prev_value = 0;
    for c in s.chars().rev() {
        let value = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => return -1, // invalid character
        };
        if value < prev_value {
            result -= value;
        } else {
            result += value;
        }
        prev_value = value;
    }
    result
}
fn int_to_roman(mut num: i32) -> String {
    let values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let symbols = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
    let mut result = String::new();
    for i in 0..values.len() {
        while  num >= values[i] {
            result.push_str(symbols[i]);
            num -= values[i];
        }
    }
    result
}



fn main() {
    println!("{}", roman_to_int("III")); // 3
    println!("{}", roman_to_int("IV")); // 4
    println!("{}", roman_to_int("IX")); // 9
    println!("{}", roman_to_int("LVIII")); // 58
    println!("{}", roman_to_int("MCMXCIV")); // 1994
    println!("{}", int_to_roman(3)); // III
    println!("{}", int_to_roman(4)); // IV
    println!("{}", int_to_roman(9)); // IX
    println!("{}", int_to_roman(58)); // LVIII
    println!("{}", int_to_roman(1994));
}