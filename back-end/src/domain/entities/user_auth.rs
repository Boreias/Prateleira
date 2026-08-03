use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserAuth {
    id: Uuid,
    username: String,
    email: String,
    password: String,
    salt: String,
    country: String,
    is_email_verified: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    deleted_at: Option<NaiveDateTime>
}

impl UserAuth {
    pub fn new(
        id: Uuid,
        username: String,
        email: String,
        password: String,
        salt: String,
        country: String,
        is_email_verified: bool,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
        deleted_at: Option<NaiveDateTime>
    ) -> UserAuth {
        UserAuth {
            id,
            username,
            email,
            password,
            salt,
            country,
            is_email_verified,
            created_at,
            updated_at,
            deleted_at
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_email(&self) -> String {
        self.email.clone()
    }

    pub fn get_password(&self) -> String {
        self.password.clone()
    }

    pub fn get_salt(&self) -> String {
        self.salt.clone()
    }

    pub fn get_country(&self) -> String {
        self.country.clone()
    }

    pub fn get_is_active(&self) -> bool {
        self.deleted_at.is_none()
    }

    pub fn get_is_email_verified(&self) -> bool {
        self.is_email_verified
    }

    pub fn get_created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn get_updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }

    pub fn get_deleted_at(&self) -> Option<NaiveDateTime> {
        self.deleted_at
    }

    pub fn set_email(&mut self, new_email: String) {
        self.email = new_email;
    }

    pub fn set_password(&mut self, new_password: String) {
        self.password = new_password;
    }

    pub fn set_country(&mut self, new_country: String) {
        self.country = new_country;
    }

    pub fn set_is_email_verified(&mut self, new_is_email_verified: bool) {
        self.is_email_verified = new_is_email_verified;
    }

    pub fn set_updated_at(&mut self, new_updated_at: NaiveDateTime) {
        self.updated_at = new_updated_at;
    }

    pub fn set_deleted_at(&mut self, new_deleted_at: Option<NaiveDateTime>) {
        self.deleted_at = new_deleted_at;
    }
}