enum Access {
    Full,
}

fn numbers() -> (i32, i32, i32) {
    (10, 11, 12)
}

fn checking_y(car_coor: (i32, i32)) -> (i32) {
    let (x, y) = car_coor;

    if (y > 5) {
        println!("Coor y: ({:}) is greater than 5.", y);
    } else {
        println!("Coor y: ({:}) is smaller than 5.", y);
    }

    return y;
}

fn get_coor() -> (i32, i32) {
    (20, 4)
}

fn main() {
    let nums = numbers();
    let (a, b, c) = numbers(); //destructuring

    println!("Printing using variable: {:?}, {:?}, {:?}", nums.0, nums.1, nums.2);
    println!("Printing using destructured variables: {:?}, {:?}, {:?}", a, b, c);

    checking_y((10, 11));

    let (x, y) = get_coor();

    if y > 5 {
        println!("Y is greater than 5.");
    } else if y < 5 {
        println!("Y is less than 5.");
    } else {
        println!("Y is equal to 5.");
    }
}