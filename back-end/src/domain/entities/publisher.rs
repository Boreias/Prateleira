use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::entities::book::Book;



#[derive(Debug, Serialize, Deserialize)]
pub struct Publisher {
    pub _id: Uuid,
    pub _name: String,
    pub _site: String,
    pub _email: String,
    pub _avatar: String,
    pub _books: Vec<Book>
}

impl Publisher {
    pub fn new(
        _id: Uuid,
        _name: String,
        _site: String,
        _email: String,
        _avatar: String,
        _books: Vec<Book>
    ) -> Self {
        Publisher {
            _id,
            _name,
            _site,
            _email,
            _avatar,
            _books
        }
    }
}