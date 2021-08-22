use crate::model::*;
use mongodb::{
    bson::{doc, Document},
    error::Error,
};
pub struct mongo_service;

impl mongo_service {
    pub fn view_one_todo(task_id: i64) -> Result<Document, Error> {
        Ok(doc! {"Placeholder"})
    }
    pub fn create_one_todo(task: InsertableUpdatableTodo) -> Result<Document, Error> {
        Ok(doc! {"Placeholder"})
    }
    pub fn update_one_todo(task: InsertableUpdatableTodo) -> Result<Document, Error> {
        Ok(doc! {"Placeholder"})
    }
    pub fn delete_one_todo(task_id: i64) -> Result<bool, Error> {
        Ok(true)
    }
}
