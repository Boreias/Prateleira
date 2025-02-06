use crate::domain::entities::gender::Gender;
use crate::domain::irepositories::gender_irepository::GenderIRepository;
use async_trait::async_trait;

pub struct GenderService<T: GenderIRepository> {
    repository: T,
}

impl<T: GenderIRepository> GenderService<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn register_gender(&self, name: String) -> Result<(), String> {
        let gender = Gender::new(name);
        self.repository.create_gender(&gender).await
    }
}