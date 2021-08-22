use rocket::{
    delete, get,
    http::Status,
    post, put,
    response::status,
    serde::json::{json, Value},
};

use crate::model::*;

#[get("/view-one-todo")]
fn view_one_todo() -> status::Custom<Value> {
    status::Custom(Status::Ok, json!({"Success":true}))
}

#[post("/create-one-todo")]
fn create_one_todo() -> status::Custom<Value> {
    status::Custom(Status::Ok, json!({"Success":true}))
}

#[put("/update-one-todo")]
fn update_one_todo() -> status::Custom<Value> {
    status::Custom(Status::Ok, json!({"Success":true}))
}

#[delete("/delete-one-todo")]
fn delete_one_todo() -> status::Custom<Value> {
    status::Custom(Status::Ok, json!({"Success":true}))
}
