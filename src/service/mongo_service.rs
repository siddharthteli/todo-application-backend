use crate::model::*;
use mongodb::{
    bson,
    bson::{doc, Document},
    error::Error,
    results::{DeleteResult, InsertOneResult, UpdateResult},
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
        let collection = Mongo_service::connection().await?;
        let task_id_document = doc! {"task_id":task_id};
        let todo_document_result = collection.find_one(task_id_document, None).await?.unwrap();
        Ok(todo_document_result)
    }
    pub async fn create_one_todo(task: InsertableUpdatableTodo) -> Result<InsertOneResult, Error> {
        let collection = Mongo_service::connection().await?;
        let task_document = bson::to_document(&task)?;
        let todo_document_result = collection.insert_one(task_document, None).await?;
        Ok(todo_document_result)
    }
    pub async fn update_one_todo(task: InsertableUpdatableTodo) -> Result<UpdateResult, Error> {
        let collection = Mongo_service::connection().await?;
        let document_filter = bson::to_document(&task.task_id)?;
        let task_document = bson::to_document(&task)?;
        let todo_document_result = collection
            .update_one(document_filter, task_document, None)
            .await?;
        Ok(todo_document_result)
    }
    pub async fn delete_one_todo(task_id: i64) -> Result<DeleteResult, Error> {
        let collection = Mongo_service::connection().await?;
        let document_filter = doc! {("task_id":task_id)};
        let todo_document_result = collection.delete_one(document_filter, None).await?;

        Ok(todo_document_result)
    }
}
