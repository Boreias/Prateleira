use async_trait::async_trait;
use crate::domain::entities::author::Author;
use crate::domain::entities::book::Book;


#[async_trait]
pub trait IAuthorRepository {
    async fn create_author(
        &self,
        name: String,
        avatar: String,
        user_id: i32,
        books_id: Option<Vec<i32>>
    ) -> Result<(), String>;

    async fn get_author_by_id(&self, id: i32) -> Result<Author, String>;

    async fn get_author_by_name(&self, name: String, skip: i32, page_size: i32) -> Result<Option<Vec<Author>>, String>;

    async fn get_authors_by_book(&self, book_id: i32, skip: i32, page_size: i32) -> Result<Option<Vec<Author>>, String>;

    async fn get_authors_by_publisher(&self, publisher_id: i32, skip: i32, page_size: i32) -> Result<Option<Vec<Author>>, String>;

    async fn more_popular_author(
        &self,
        skip: i32,
        page_size: i32
    ) -> Result<Option<Vec<Author>>, String>; // TODO: autores com os livros mais lidos

    async fn best_valuated_author(
        &self,
        skip: i32,
        page_size: i32
    ) -> Result<Option<Vec<Author>>, String>;// TODO: autores com os livros melhor avaliados

    async fn alter_author(
        &mut self,
        id: String,
        name: String,
        avatar: String,
        user_id: i32,
        books_id: Option<Vec<i32>>
    ) -> Result<(), String>;

    async fn delete_author(&self, id: String) -> Result<(), String>;
}