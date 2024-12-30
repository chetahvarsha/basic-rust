fn reverse(x: i32) -> i32 {
    const MAX: i32 = 0x7FFFFFFF; // Maximum value for a 32-bit signed integer
    const MIN: i32 = -0x80000000; // Minimum value for a 32-bit signed integer
    
    let mut num = x;
    let mut reversed = 0;

    while num != 0 {
        let digit = num % 10;
        num /= 10;

        // Check for overflow before it happens
        if digit > MAX % 10 || (reversed == MAX / 10 && digit > MAX % 10) {
            return 0;
        }
        if digit < MIN % 10 || (reversed == MIN / 10 && digit < MIN % 10) {
            return 0;
        }
        
        reversed = reversed * 10 + digit;
    }

    reversed
}

fn main() {
    let x = 123;
    println!("Reversed: {}", reverse(x)); // Output: Reversed: 321

    let y = -123;
    println!("Reversed: {}", reverse(y)); // Output: Reversed: -321

    let z = 1534236469;
    println!("Reversed: {}", reverse(z)); // Output: Reversed: 0 (overflow case)
}

