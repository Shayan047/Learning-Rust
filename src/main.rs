struct Grocery {
    id: i32,
    item: String,
    price: i32,
    quantity: i32
}

fn display_quantity(grocery: &Grocery) {
    println!("{}", grocery.quantity);
}

fn display_id(grocery: &Grocery) {
    println!("{}", grocery.id);
}


fn main() {
    let grocery = Grocery {
        id: 1,
        item: String::from("Milk"),
        price: 3,
        quantity: 2,
    };

    display_quantity(&grocery);
    display_id(&grocery);
}
