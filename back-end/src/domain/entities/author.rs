use uuid::Uuid;

use crate::domain::entities::book::Book;


#[derive(Clone)]
pub struct Author {
    id: Uuid,
    name: String,
    avatar: String,
    books: Option<Vec<Book>>
}


impl Author {
    pub fn new(id: Uuid, name: String, avatar: String, books: Option<Vec<Book>>) -> Author {
        Author {
            id,
            name,
            avatar,
            books
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_avatar(&self) -> String {
        self.avatar.clone()
    }

    pub fn get_books(&self) -> Option<Vec<Book>> {
        self.books.clone()
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn set_avatar(&mut self, new_avatar: String) {
        self.avatar = new_avatar;
    }

    pub fn set_books(&mut self, new_books: Option<Vec<Book>>) {
        self.books = new_books;
    }
}