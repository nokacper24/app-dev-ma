use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Author {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub birth_year: i32,
}

impl Author {
    pub fn new(id: i32, first_name: String, last_name: String, birth_year: i32) -> Author {
        Author {
            id,
            first_name,
            last_name,
            birth_year,
        }
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn first_name(&self) -> &String {
        &self.first_name
    }
    pub fn last_name(&self) -> &String {
        &self.last_name
    }
    pub fn birth_year(&self) -> &i32 {
        &self.birth_year
    }
    pub fn id_mut(&mut self) -> &mut i32 {
        &mut self.id
    }
    pub fn first_name_mut(&mut self) -> &mut String {
        &mut self.first_name
    }
    pub fn last_name_mut(&mut self) -> &mut String {
        &mut self.last_name
    }
    pub fn birth_year_mut(&mut self) -> &mut i32 {
        &mut self.birth_year
    }
}