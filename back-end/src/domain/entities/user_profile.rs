use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDate;


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserProfile {
    id: Uuid,
    name: String,
    bio: Option<String>,
    birth_date: NaiveDate,
    avatar: Option<String>
}

impl UserProfile {
    pub fn new(
        id: Uuid,
        name: String,
        bio: Option<String>,
        birth_date: NaiveDate,
        avatar: Option<String>
    ) -> UserProfile {
        UserProfile {
            id,
            name,
            bio,
            birth_date,
            avatar
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_bio(&self) -> Option<String> {
        self.bio.clone()
    }

    pub fn get_birth_date(&self) -> NaiveDate {
        self.birth_date
    }

    pub fn get_avatar(&self) -> Option<String> {
        self.avatar.clone()
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn set_bio(&mut self, new_bio: Option<String>) {
        self.bio = new_bio;
    }

    pub fn set_birth_date(&mut self, new_birth_date: NaiveDate) {
        self.birth_date = new_birth_date;
    }
}