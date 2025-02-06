use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub _id: Uuid,
    pub _title: String,
    pub _subtitle: String,
    pub _authors: Vec<Uuid>,
    pub _publisher: Uuid,
    pub _series_collection: i32,
    pub _volume: i32,
    pub _edition: i32,
    pub _publication_year: i32,
    pub _pages: i32,
    pub _language: String,
    pub _isbn: String,
    pub _synopsis: String,
    pub _cover: String,
    // pub _gender: String,
}


impl Book {
    pub fn new(
        _id: Uuid,
        _title: String,
        _subtitle: String,
        _authors: Vec<Uuid>,
        _publisher: Uuid,
        _series_collection: i32,
        _volume: i32,
        _edition: i32,
        _publication_year: i32,
        _pages: i32,
        _language: String,
        _isbn: String,
        _synopsis: String,
        _cover: String
    ) -> Self {
        Book {
            _id,
            _title,
            _subtitle,
            _authors,
            _publisher,
            _series_collection,
            _volume,
            _edition,
            _publication_year,
            _pages,
            _language,
            _isbn,
            _synopsis,
            _cover
        }
    }
}