/* 
Objective: Design a basic Library Management System (LMS) using an object-oriented programming language 
of your choice (e.g., Java, Python, C#). The system should allow users to manage books and patrons in a small library.

Requirements:

    Functional Requirements:
        Book Management:
            Add new books to the library.
            Remove books from the library.
            Update book information (title, author, and ISBN).
            Search for books by title, author, or ISBN.
        Patron Management:
            Register new patrons.
            Remove patrons.
            Update patron information (name and patron ID).
            Check out and return books for patrons.
*/
use std::collections::HashMap;

struct Book {
    title: String,
    author: String,
    isbn: u64,
}

impl Book {
    fn set_title(&self) -> &str {
        &self.title
    }
    fn set_author(&self) -> &str {
        &self.author
    }
    fn set_isbn(&self) -> &str {
        &self.isbn 
    }
}


fn main() {
    let mut library = HashMap::new();

    library.insert
}   
