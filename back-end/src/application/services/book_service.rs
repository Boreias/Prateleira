use crate::domain::entities::book::Book;
use crate::domain::irepositories::book_irepository::BookIRepository;
use async_trait::async_trait;

pub struct BookService<T: BookIRepository> {
    repository: T,
}

impl<T: BookIRepository> BookService<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn register_book(
        &self, 
        title: String,
        subtitle: String,
        authors: Vec<i32>,
        publisher: i32,
        series_collection: i32,
        volume: i32,
        edition: i32,
        publication_year: i32,
        pages: i32,
        language: String,
        isbn: String,
        synopsis: String,
        cover: String
    ) -> Result<(), String> {
        let book = Book::new(
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
            synopsis,
            cover
        );
        self.repository.create_book(&book).await
    }
}