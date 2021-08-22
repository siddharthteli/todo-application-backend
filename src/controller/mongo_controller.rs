use rocket::{
    delete, get,
    http::Status,
    post, put,
    response::status,
    serde::json::{json, Value},
};

use crate::model::*;

#[get("/view-one-todo")]
pub fn view_one_todo() -> status::Custom<Value> {
    status::Custom(Status::Ok, json!({"Success":true}))
}

#[post("/create-one-todo")]
pub fn create_one_todo() -> status::Custom<Value> {
    status::Custom(Status::Ok, json!({"Success":true}))
}

#[put("/update-one-todo")]
pub fn update_one_todo() -> status::Custom<Value> {
    status::Custom(Status::Ok, json!({"Success":true}))
}

#[delete("/delete-one-todo")]
pub fn delete_one_todo() -> status::Custom<Value> {
    status::Custom(Status::Ok, json!({"Success":true}))
}
