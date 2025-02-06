use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: Uuid,
    pub _name: String,
    pub _nickname: String,
    pub _email: String,
    pub _password: String,
    pub _registrationData: chrono::DateTime<chrono::Utc>,
    pub _avatar: String,
}


impl User {
    pub fn new(
        _id: Uuid,
        _name: String,
        _nickname: String,
        _email: String,
        _password: String,
        _registrationData: chrono::DateTime<chrono::Utc>,
        _avatar: String,
    ) -> Self {
        User {
            _id,
            _name,
            _nickname,
            _email,
            _password,
            _registrationData,
            _avatar,
        }
    }
}