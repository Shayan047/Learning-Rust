struct Book {
    pages: i32,
    rating: i32
}

fn display_pages(book: &Book) {
    println!("{}", book.pages);
}

fn display_rating(book: &Book) {
    println!("{}", book.rating);
}

fn main() {
    let book = Book {
        pages: 150,
        rating: 4
    };

    display_pages(&book);
    display_rating(&book);
}