enum Flavor {
    Orange,
    Pineapple,
    Chocolate
}

struct Drink {
    flavor: Flavor,
    fluid: i32,
}

fn print_flavor(flavor: Flavor) {
    match flavor {
        Flavor::Orange => println!("Flavor is Orange."),
        Flavor::Pineapple => println!("Flavor is Pineapple."),
        Flavor::Chocolate => println!("FLavor is Chocolate.")
    }
}

fn print_fluid_ounces(ounces: i32) {
    println!("Ounces: {:?}oz", ounces);
}

fn main() {
    let drink = Drink {
        flavor: Flavor::Orange,
        fluid: 20,
    };

    print_flavor(drink.flavor);
    print_fluid_ounces(drink.fluid);
}