pub mod book;
use std::collections::HashMap;

use book::Book;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A collection of books
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Books {
    books: HashMap<String, Book>,
}
impl Books {
    /// Create a new collection of books
    pub fn new() -> Self {
        Books {
            books: HashMap::new(),
        }
    }

    /// Get all books in the collection, as a vector
    pub fn get_all(&self) -> Vec<Book> {
        self.books.values().cloned().collect()
    }

    /// Add a book to the collection
    pub fn add(&mut self, book: Book) {
        self.books.insert(book.id().clone(), book);
    }

    /// Get a book by its id
    pub fn get(&self, id: String) -> Option<Book> {
        self.books.get(&id).cloned()
    }

    /// Remove a book from the collection
    pub fn remove(&mut self, id: &str) -> Option<Book> {
        self.books.remove(id)
    }

    /// Get the number of books in the collection
    pub fn count(&self) -> usize {
        self.books.len()
    }
}

impl Default for Books {
    /// Create a new collection of books
    fn default() -> Self {
        Self::new()
    }
}
