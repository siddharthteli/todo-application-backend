use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

//schema of todo mongodb document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub task_id: u64,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertableUpdatableTodo {
    pub task_id: i64,
    pub title: String,
    pub description: String,
}
