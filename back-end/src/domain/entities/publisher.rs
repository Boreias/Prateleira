use serde::Serialize;

use crate::domain::entities::book::Book;


#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Publisher {
    id: i32,
    name: String,
    site: Option<String>,
    email: Option<String>,
    avatar: Option<String>,
    books: Option<Vec<Book>>
}


impl Publisher {
    pub fn new(
        id: i32,
        name: String,
        site: Option<String>,
        email: Option<String>,
        avatar: Option<String>,
        books: Option<Vec<Book>>
    ) -> Publisher {
        Publisher {
            id,
            name,
            site,
            email,
            avatar,
            books
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_site(&self) -> Option<String> {
        self.site.clone()
    }

    pub fn get_email(&self) -> Option<String> {
        self.email.clone()
    }

    pub fn get_avatar(&self) -> Option<String> {
        self.avatar.clone()
    }

    pub fn get_books(&self) -> Option<Vec<Book>> {
        self.books.clone()
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn set_site(&mut self, new_site: Option<String>) {
        self.site = new_site;
    }

    pub fn set_email(&mut self, new_email: Option<String>) {
        self.email = new_email;
    }

    pub fn set_avatar(&mut self, new_avatar: Option<String>) {
        self.avatar = new_avatar;
    }

    pub fn set_books(&mut self, new_books: Option<Vec<Book>>) {
        self.books = new_books;
    }
}