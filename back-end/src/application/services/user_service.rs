use crate::domain::entities::user::User;
use crate::domain::irepositories::user_irepository::UserIRepository;
use async_trait::async_trait;

pub struct UserService<T: UserIRepository> {
    repository: T,
}

impl<T: UserIRepository> UserService<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn register_user(&self, name: String, nickname: String, email: String, password: String, registrationData: chrono::DateTime<chrono::Utc>, avatar: String) -> Result<(), String> {
        if self.repository.get_user_by_email(&email).await?.is_some() {
            return Err("User already exists".to_string());
        }

        let user = User::new(name, nickname, email, password, registrationData, avatar);
        self.repository.create_user(&user).await
    }
}