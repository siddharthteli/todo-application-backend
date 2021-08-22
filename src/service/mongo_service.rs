use crate::model::*;
use mongodb::{
    bson::{doc, Document},
    error::Error,
    Client, Collection,
};
pub struct Mongo_service;

impl Mongo_service {
    pub async fn connection() -> Result<Collection, Error> {
        let client = Client::with_uri_str("mongodb://localhost:27017").await?;
        let collection = client.database("todo").collection("todo_collection");
        Ok(collection)
    }
    pub async fn view_one_todo(task_id: i64) -> Result<Document, Error> {
        let collection=Mongo_service::connection().await?;
        let task_id_document = doc! {"task_id":task_id};
        let todo_document=collection.find_one(task_id_document, None).await?.unwrap();
        Ok(todo_document)
    }
    pub async sfn create_one_todo(task: InsertableUpdatableTodo) -> Result<Document, Error> {
        Ok(doc! {"Placeholder"})
    }
    pub async fn update_one_todo(task: InsertableUpdatableTodo) -> Result<Document, Error> {
        Ok(doc! {"Placeholder"})
    }
    pub async fn delete_one_todo(task_id: i64) -> Result<bool, Error> {
        Ok(true)
    }
}
