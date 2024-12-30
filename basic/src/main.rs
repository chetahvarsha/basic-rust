fn arithmetic_operators(num: i32,num_1: i32){
   let mut result = num + num_1;
   println!("sum = reusult {}",result);
   result = num-num_1;
   println!("difference = result {}",result);
   result = num*num_1;
   println!("mult = result {}",result);
}

fn main() {
    let a = 22;
    let b = 33;
    arithmetic_operators(a,b);
    let a = b as f32 /10.0 ;
    println!("div = result {}",a);
if a != 0.0 {
    println!("a is a float");
}else{
    println!("a is not a float");
}

}
