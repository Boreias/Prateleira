use uuid::Uuid;
use chrono::NaiveDate;


pub struct User {
    id: Uuid,
    name: String,
    nickname: String,
    email: String,
    password: String,
    salt: String,
    birth_date: NaiveDate,
    registration_date: NaiveDate,
    avatar: String
}

impl User{
    pub fn new(
        id: Uuid,
        name: String,
        nickname: String,
        email: String,
        password: String,
        salt: String,
        birth_date: NaiveDate,
        registration_date: NaiveDate,
        avatar: String
    ) -> User {
        User {
            id,
            name,
            nickname,
            email,
            password,
            salt,
            birth_date,
            registration_date,
            avatar
        }
    }
    
    pub fn get_id(&self) -> Uuid {
        self.id.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_nickname(&self) -> String {
        self.nickname.clone()
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

    pub fn get_birth_date(&self) -> NaiveDate {
        self.birth_date.clone()
    }

    pub fn get_registration_date(&self) -> NaiveDate {
        self.registration_date.clone()
    }

    pub fn get_avatar(&self) -> String {
        self.avatar.clone()
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn set_nickname(&mut self, new_nickname: String) {
        self.nickname = new_nickname;
    }

    pub fn set_password(&mut self, new_password: String) {
        self.password = new_password;
    }

    pub fn set_birth_date(&mut self, new_birth_date: NaiveDate) {
        self.birth_date = new_birth_date;
    }

    pub fn set_registration_date(&mut self, new_registration_date: NaiveDate) {
        self.registration_date = new_registration_date;
    }

    pub fn set_avatar(&mut self, new_avatar: String) {
        self.avatar = new_avatar;
    }
}