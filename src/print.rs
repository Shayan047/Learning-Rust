fn first_name() {
    println!("Shayan");
}

fn last_name() {
    println!("Shaikh");
}

fn sub(a: i32, b: i32) -> i32 {
    return a - b;
}

fn main() {
    first_name();
    last_name();
    println!("Above is my name displayed using functions");
    println!(sub(10, 20));
}
