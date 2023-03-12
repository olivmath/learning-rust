use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

lazy_static! {
    static ref DB: Arc<Mutex<HashMap<String, Book>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct Book {
    pub title: String,
    pub year: u16,
}

impl Book {
    fn new(title: &str, year: u16) -> Self {
        Book {
            title: String::from(title),
            year,
        }
    }
    pub fn default() -> Self {
        Book {
            title: "".into(),
            year: 0,
        }
    }
}

impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.title, self.year)
    }
}

pub struct Library {
    books: Arc<Mutex<HashMap<String, Book>>>,
}

impl Library {
    pub fn new() -> Library {
        Library {
            books: Arc::clone(&DB),
        }
    }

    pub fn len(&self) -> usize {
        self.books.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.books.lock().unwrap().is_empty()
    }

    pub fn add_book(&mut self, id: String, book: Book) -> Option<Book> {
        self.books.lock().unwrap().insert(id, book)
    }

    pub fn modify_book(&mut self, id: String, book: Book) -> Option<Book> {
        self.books.lock().unwrap().insert(id, book)
    }

    pub fn remove_book(&mut self, id: String) -> Option<Book> {
        self.books.lock().unwrap().remove(&id)
    }

    pub fn all_books(&self) -> Vec<Book> {
        self.books
            .lock()
            .unwrap()
            .iter()
            .map(|(_, v)| v.clone())
            .collect::<Vec<Book>>()
    }

    pub fn get_book(&self, id: String) -> Option<Book> {
        self.books.lock().unwrap().get(&id).cloned()
    }
}
