use crate::model::*;
use mongodb::{
    bson,
    bson::{doc, oid::ObjectId, Document},
    error::Error,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};
use rocket::{futures::TryStreamExt, serde::json::Value};
pub struct Mongo_service;

impl Mongo_service {
    pub async fn connection() -> Result<Collection, Error> {
        let client = Client::with_uri_str("mongodb://localhost:27017").await?;
        let collection = client.database("todo").collection("todo_collection");
        Ok(collection)
    }
    pub async fn view_all_todo() -> Result<Vec<Document>, Error> {
        let collection = Mongo_service::connection().await?;
        let cursor = collection.find(None, None).await?;
        let document_list: Vec<Document> = cursor.try_collect().await?;
        Ok(document_list)
    }
    pub async fn view_one_todo(task_id: ObjectId) -> Result<Document, Error> {
        let collection = Mongo_service::connection().await?;
        let task_id_document = doc! {"_id":task_id};
        let todo_document_result = collection.find_one(task_id_document, None).await?.unwrap();
        Ok(todo_document_result)
    }
    pub async fn create_one_todo(task: InsertableTodo) -> Result<InsertOneResult, Error> {
        let collection = Mongo_service::connection().await?;
        let task_document = bson::to_document(&task)?;
        let todo_document_result = collection.insert_one(task_document, None).await?;
        Ok(todo_document_result)
    }
    pub async fn update_one_todo(task: UpdatableTodo) -> Result<UpdateResult, Error> {
        let collection = Mongo_service::connection().await?;
        let document_filter = doc! {"_id":task.task_id};
        let task_document = doc! {"title":task.title,"description":task.description};
        let todo_document_result = collection
            .update_one(document_filter, task_document, None)
            .await?;
        Ok(todo_document_result)
    }
    pub async fn delete_one_todo(task_id: ObjectId) -> Result<DeleteResult, Error> {
        let collection = Mongo_service::connection().await?;
        let document_filter = doc! {"_id":task_id};
        let todo_document_result = collection.delete_one(document_filter, None).await?;

        Ok(todo_document_result)
    }
}
