enum Access {
    Full,
}

fn numbers() -> (i32, i32, i32) {
    (10, 11, 12)
}

fn main() {
    let nums = numbers();
    let (a, b, c) = numbers(); //destructuring

    println!("Printing using variable: {:?}, {:?}, {:?}", nums.0, nums.1, nums.2);
    println!("Printing using destructured variables: {:?}, {:?}, {:?}", a, b, c);
}