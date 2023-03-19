#![allow(unused)]

#[derive(Clone)]
struct Book {
    title: String,
    year: u16,
}

impl Book {
    // This is a constructor, used below.
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// This makes it possible to print Book values with {}.
impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.title, self.year)
    }
}

struct Library {
    books: Vec<Book>,
}
impl Library {
    pub fn new(books: Vec<Book>) -> Library {
        Library { books }
    }

    pub fn empty() -> Library {
        Library { books: vec![] }
    }

    pub fn len(&self) -> usize {
        self.books.len()
    }

    pub fn is_empty(&self) -> bool {
       self.books.is_empty().clone()
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn print_books(&self) {
        self.books.iter().for_each(|book| println!("📖 {book}"));
    }

    pub fn oldest_book<'library_lifetime>(&'library_lifetime self) -> Option<&'library_lifetime Book> {
        self.books.iter().min_by_key(|book| book.year)
    }
}


fn main() {
    let mut library = Library::empty();

    println!("📚 Our library is empty: {}", library.is_empty());

    let favorite_book = Book::new("Lord of the Rings", 1954);
    library.add_book(favorite_book.clone());
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));

    library.print_books();

    match library.oldest_book() {
        Some(book) => println!("My oldest book is {book}"),
        None => println!("My library is empty!"),
    }

    println!("Our library has {} books", library.len());
    for book in library.books {
        println!("{book}");
    }
}
