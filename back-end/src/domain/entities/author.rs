use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::entities::book::Book;


#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub _id: Uuid,
    pub _name: String,
    pub _avatar: String,
    pub _books: Vec<Book>,
}

impl Author {
    pub fn new(
        _id: Uuid,
        _name: String,
        _avatar: String,
        _books: Vec<Book>
    ) -> Self {
        Author {
            _id,
            _name,
            _avatar,
            _books
        }
    }
}