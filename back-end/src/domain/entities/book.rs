use serde::Serialize;

use crate::domain::entities::author::Author;
use crate::domain::entities::publisher::Publisher;
use crate::domain::entities::gender::Gender;


#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Book {
    id: i32,
    title: String,
    subtitle: Option<String>,
    authors: Vec<Author>,
    publisher: Publisher,
    series_collection: Option<i8>,
    volume: Option<i8>,
    edition: Option<i8>,
    publication_year: i16,
    pages: i32,
    language: String,
    isbn: String,
    gender: Vec<Gender>,
    synopsis: String,
    cover: String
}

impl Book {
    pub fn new(
        id: i32,
        title: String,
        subtitle: Option<String>,
        authors: Vec<Author>,
        publisher: Publisher,
        series_collection: Option<i8>,
        volume: Option<i8>,
        edition: Option<i8>,
        publication_year: i16,
        pages: i32,
        language: String,
        isbn: String,
        gender: Vec<Gender>,
        synopsis: String,
        cover: String
    ) -> Book {
        Book {
            id,
            title,
            subtitle,
            authors,
            publisher,
            series_collection,
            volume,
            edition,
            publication_year,
            pages,
            language,
            isbn,
            gender,
            synopsis,
            cover
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id.clone()
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_subtitle(&self) -> Option<String> {
        self.subtitle.clone()
    }

    pub fn get_authors(&self) -> Vec<Author> {
        self.authors.clone()
    }

    pub fn get_publisher(&self) -> Publisher {
        self.publisher.clone()
    }

    pub fn get_series_collection(&self) -> Option<i8> {
        self.series_collection.clone()
    }

    pub fn get_volume(&self) -> Option<i8> {
        self.volume.clone()
    }

    pub fn get_edition(&self) -> Option<i8> {
        self.edition.clone()
    }

    pub fn get_publication_year(&self) -> i16 {
        self.publication_year.clone()
    }

    pub fn get_pages(&self) -> i32 {
        self.pages.clone()
    }

    pub fn get_language(&self) -> String {
        self.language.clone()
    }

    pub fn get_isbn(&self) -> String {
        self.isbn.clone()
    }

    pub fn get_gender(&self) -> Vec<Gender> {
        self.gender.clone()
    }

    pub fn get_synopsis(&self) -> String {
        self.synopsis.clone()
    }

    pub fn get_cover(&self) -> String {
        self.cover.clone()
    }

    pub fn set_title(&mut self, new_title: String) {
        self.title = new_title;
    }

    pub fn set_subtitle(&mut self, new_subtitle: Option<String>) {
        self.subtitle = new_subtitle;
    }

    pub fn set_authors(&mut self, new_authors: Vec<Author>) {
        self.authors = new_authors;
    }

    pub fn set_publisher(&mut self, new_publisher: Publisher) {
        self.publisher = new_publisher;
    }

    pub fn set_series_collection(&mut self, new_series_collection: Option<i8>) {
        self.series_collection = new_series_collection;
    }

    pub fn set_volume(&mut self, new_volume: Option<i8>) {
        self.volume = new_volume;
    }

    pub fn set_edition(&mut self, new_edition: Option<i8>) {
        self.edition = new_edition;
    }

    pub fn set_publication_year(&mut self, new_publication_year: i16) {
        self.publication_year = new_publication_year;
    }

    pub fn set_pages(&mut self, new_pages: i32) {
        self.pages = new_pages;
    }

    pub fn set_language(&mut self, new_language: String) {
        self.language = new_language;
    }

    pub fn set_isbn(&mut self, new_isbn: String) {
        self.isbn = new_isbn;
    }

    pub fn set_gender(&mut self, new_gender: Vec<Gender>) {
        self.gender = new_gender;
    }

    pub fn set_synopsis(&mut self, new_synopsis: String) {
        self.synopsis = new_synopsis;
    }

    pub fn set_cover(&mut self, new_cover: String) {
        self.cover = new_cover;
    }
}