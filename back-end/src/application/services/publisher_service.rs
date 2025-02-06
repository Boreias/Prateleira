use crate::domain::entities::publisher::Publisher;
use crate::domain::irepositories::publisher_irepository::PublisherIRepository;
use async_trait::async_trait;

pub struct PublisherService<T: PublisherIRepository> {
    repository: T,
}

impl<T: PublisherIRepository> PublisherService<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn register_publisher(&self, name: String, site: String, email: String, avatar: String) -> Result<(), String> {
        let publisher = Publisher::new(name, site, email, avatar);
        self.repository.create_publisher(&publisher).await
    }
}