use async_trait::async_trait;
use crate::domain::entities::publisher::Publisher;

#[async_trait]
pub trait PublisherIRepository {
    async fn get_all_publishers(&self) -> Result<Vec<Publisher>, String>;
    async fn get_publisher_by_id(&self, id: i32) -> Result<Publisher, String>;
    async fn get_publisher_by_name(&self, name: &str) -> Result<Publisher, String>;
    async fn create_publisher(&self, publisher: Publisher) -> Result<(), String>;
}