#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gender {
    id: i32,
    name: String
}


impl Gender {
    pub fn new(id: i32, name: String) -> Gender {
        Gender { id, name }
    }

    pub fn get_id(&self) -> i32 {
        self.id.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }
}