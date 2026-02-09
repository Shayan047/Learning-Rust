enum Access {
    Admin,
    Manager,
    User,
    Guest
}

fn print_result(value: bool) {
    match value {
        true => println!("Its big."),
        false => println!("Its small.")
    }
}

fn main() {
    let access_level = Access::Guest;
    let can_access = match access_level {
        Access::Admin => true,
        _ => false,
    };

    println!("{}", can_access);

    let number = 99;
    let gt = number > 100;

    print_result(gt);
}