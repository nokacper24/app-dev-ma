use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema)]
pub struct Book {
    id: String,
    title: String,
    year: u16,
    number_pages: u32,
}

impl Book {
    pub fn new(id: String, title: String, year: u16, number_pages: u32) -> Book {
        Book {
            id,
            title,
            year,
            number_pages,
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }
    pub fn title(&self) -> &String {
        &self.title
    }
    pub fn year(&self) -> &u16 {
        &self.year
    }
    pub fn number_pages(&self) -> &u32 {
        &self.number_pages
    }
    pub fn id_mut(&mut self) -> &mut String {
        &mut self.id
    }
    pub fn title_mut(&mut self) -> &mut String {
        &mut self.title
    }
    pub fn year_mut(&mut self) -> &mut u16 {
        &mut self.year
    }
    pub fn number_pages_mut(&mut self) -> &mut u32 {
        &mut self.number_pages
    }
}
