use rocket::{
    delete, get,
    http::Status,
    post, put,
    response::status,
    serde::json::{json, Json, Value},
};

use crate::model::*;

#[get("/view-one-todo", data = "<task_id>")]
pub fn view_one_todo(task_id: Json<i64>) -> status::Custom<Value> {
    status::Custom(Status::Ok, json!({"Success":true}))
}

#[post("/create-one-todo", data = "<task>")]
pub fn create_one_todo(task: Json<InsertableUpdatableTodo>) -> status::Custom<Value> {
    status::Custom(Status::Ok, json!({"Success":true}))
}

#[put("/update-one-todo", data = "<task>")]
pub fn update_one_todo(task: Json<InsertableUpdatableTodo>) -> status::Custom<Value> {
    status::Custom(Status::Ok, json!({"Success":true}))
}

#[delete("/delete-one-todo", data = "<task_id>")]
pub fn delete_one_todo(task_id: Json<i64>) -> status::Custom<Value> {
    status::Custom(Status::Ok, json!({"Success":true}))
}
