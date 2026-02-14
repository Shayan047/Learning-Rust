enum Light {
    White,
    Yellow,
}

fn display_light(light: &Light) {
    match light {
        Light::White => println!("White"),
        Light::Yellow => println!("Yellow"),
    }
}

fn main() {
    let light = Light::Yellow;

    //Ownership explanation:

    //display_light(light);
    //display_light(light);

    //the second function in the above line will be not executed because of the ownership rules of Rust.
    //initially when the light variable was created it was owned by the main function but when it was frist passed into the display_light
    //function it was moved to that function and the main function lost the ownership of that variable
    //after the display_light function is executed the light variable is deleted so it no longer exists in the main function
    //hence we cannot call the display_light function again using the light function

    //Solution: we use borrowing which allows us to borrow the value of variable without changing its ownership.
    //          in order words we can say it is a reference.


    display_light(&light);
    display_light(&light);

    //we use & to borrow the value in Rust.
}