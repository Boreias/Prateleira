use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::entities::book::Book;



#[derive(Debug, Serialize, Deserialize)]
pub struct Gender {
    pub _id: Uuid,
    pub _name: String,
    pub _books: Vec<Book>
}

impl Gender {
    pub fn new(
        _id: Uuid,
        _name: String,
        _books: Vec<Book>
    ) -> Self {
        Gender {
            _id,
            _name,
            _books
        }
    }
}