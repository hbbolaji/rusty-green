enum Book {
    TheRustProgrammingLanguage,
    ZeroToProduction,
    Other(String)
}

fn title(book: &Book) -> String {
    match book {
        Book::TheRustProgrammingLanguage => "The Rust Programming Language".into(),
        Book::ZeroToProduction => "Zero To Production".into(),
        Book::Other(title) => title.into()
    }
}

fn author(book: &Book) -> Option<String> {
    match book {
        Book::TheRustProgrammingLanguage => Some("Steve Klabnik and CArol Nichols".into()),
        Book::ZeroToProduction => Some("Luca Palmiere".into()),
        Book::Other(_) => None
    }
}

fn display(book: &Book) -> String {
    let title = title(&book);
    match author(&book) {
        Some(author) => format!("{} by {}", title, author),
        None => format!("{}", title)
    }
}

fn main() {
    println!("Hello, world!");
    let my_fav_book = Book::ZeroToProduction;
    let your_fav_book = Book::TheRustProgrammingLanguage;
    let his_fav_book = Book::Other(String::from("Rust In Action"));

    println!("My favorite is: {}", display(&my_fav_book));
    println!("Your favorite is {}", display(&your_fav_book));
    println!("His favorite book is {}", display(&his_fav_book));
}
