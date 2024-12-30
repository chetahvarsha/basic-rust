
fn main() {
    for num in 2..=50 {
        let mut is_prime = true;

        for i in 2..=(num as f64).sqrt() as i32 {
            if num % i == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            println!("{}", num);
        }
    }

table()
}
fn table() {
    for i in 1..=10 {
        println!("table form 1 to 10");
        for j in 1..=10 {
            print!("{:<4}", i * j); //{:<4}" part is a format specifier that controls 
        }
        println!();
    }
}
