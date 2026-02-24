use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gender {
    id: Uuid,
    name: String
}


impl Gender {
    pub fn new(id: Uuid, name: String) -> Gender {
        Gender { id, name }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }
}