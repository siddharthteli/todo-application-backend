use mongodb::{
    bson::{doc, Document},
    error::Error,
};

pub struct mongo_service;

impl mongo_service {
    pub fn view_one_todo() -> Result<Document, Error> {
        Ok(doc! {"Placeholder"})
    }
    pub fn create_one_todo() -> Result<Document, Error> {
        Ok(doc! {"Placeholder"})
    }
    pub fn update_one_todo() -> Result<Document, Error> {
        Ok(doc! {"Placeholder"})
    }
    pub fn delete_one_todo() -> Result<Document, Error> {
        Ok(doc! {"Placeholder"})
    }
}
