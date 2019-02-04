use crate::core::Entity;

struct Book {
    isbn: String
}

impl<'a> Entity<String> for Book {
    fn id(&self) -> &String {
        &self.isbn
    }
}