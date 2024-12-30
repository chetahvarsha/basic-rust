fn main() {
    let n = 5; // Number of rows for the patterns

    diamond_pattern(n);
    println!(); // Adding a line break between patterns
    half_diamond_pattern(n);
}

fn diamond_pattern(n: usize) {
    // Print the top half of the diamond
    for i in 0..n {
        // Print leading spaces
        for _ in 0..(n - i - 1) {
            print!(" ");
        }
        // Print stars
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }

    // Print the bottom half of the diamond
    for i in (0..n-1).rev() {
        // Print leading spaces
        for _ in 0..(n - i - 1) {
            print!(" ");
        }
        // Print stars
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }
}

fn half_diamond_pattern(n: usize) {
    // Print the top half of the half diamond
    for i in 1..=n {
        // Print stars
        for _ in 0..i {
            print!("*");
        }
        println!();
    }

    // Print the bottom half of the half diamond
    for i in (1..n).rev() {
        // Print stars
        for _ in 0..i {
            print!("*");
        }
        println!();
    }
}
