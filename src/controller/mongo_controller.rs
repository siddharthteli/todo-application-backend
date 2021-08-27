use mongodb::bson::oid::ObjectId;
use rocket::{
    delete, get,
    http::Status,
    options, post, put,
    response::status,
    serde::json::{json, Json, Value},
};

use crate::model::*;
use crate::service::Mongo_service;

#[get("/view-all-todo")]
pub async fn view_all_todo() -> status::Custom<Value> {
    match Mongo_service::view_all_todo().await {
        Ok(result) => status::Custom(
            Status::Ok,
            json!({ "Success":true,"message":"view_all_todo is working fine","data":result }),
        ),
        Err(e) => status::Custom(
            Status::NotFound,
            json!({  "Success":false,"message":"view_all_todo is not working","data":e.to_string() }),
        ),
    }
}

#[get("/view-one-todo/<task_id>")]
pub async fn view_one_todo(task_id: String) -> status::Custom<Value> {
    match Mongo_service::view_one_todo(ObjectId::with_string(&task_id).unwrap()).await {
        Ok(result) => status::Custom(
            Status::Ok,
            json!({ "Success":true,"message":"view_one_todo is working fine","data":result }),
        ),
        Err(e) => status::Custom(
            Status::NotFound,
            json!({  "Success":false,"message":"view_one_todo is not working","data":e.to_string() }),
        ),
    }
}

#[post("/create-one-todo", data = "<task>")]
pub async fn create_one_todo(task: Json<InsertableTodo>) -> status::Custom<Value> {
    match Mongo_service::create_one_todo(task.into_inner()).await {
        Ok(result) => status::Custom(
            Status::Ok,
            json!({ "Success":true,"message":"view_one_todo is working fine","data":result }),
        ),
        Err(e) => status::Custom(
            Status::NotFound,
            json!({  "Success":false,"message":"view_one_todo is not working","data":e.to_string() }),
        ),
    }
}

#[put("/update-one-todo", data = "<task>")]
pub async fn update_one_todo(task: Json<UpdatableTodo>) -> status::Custom<Value> {
    match Mongo_service::update_one_todo(task.into_inner()).await {
        Ok(result) => status::Custom(
            Status::Ok,
            json!({ "Success":true,"message":"view_one_todo is working fine","data":result }),
        ),
        Err(e) => status::Custom(
            Status::NotFound,
            json!({  "Success":false,"message":"view_one_todo is not working","data":e.to_string() }),
        ),
    }
}
#[options("/update-one-todo", data = "<task>")]
pub fn update_one_option(task: Json<UpdatableTodo>) -> status::Custom<Value> {
    println!("Going into options");
    status::Custom(Status::Ok, json! {"You are accessing home end point"})
}

#[options("/test", data = "<task>")]
pub fn test(task: Json<Omo>) -> status::Custom<Value> {
    println!("Going into options");
    status::Custom(Status::Ok, json! {"You are accessing home end point"})
}
#[options("/test/<data>")]
pub fn test_option(data: String) -> status::Custom<Value> {
    println!("Going into options");
    status::Custom(Status::Ok, json! {"You are accessing home end point"})
}

#[delete("/delete-one-todo/<task_id>")]
pub async fn delete_one_todo(task_id: String) -> status::Custom<Value> {
    println!("Value of task_id:{:?}", task_id);
    match Mongo_service::delete_one_todo(ObjectId::with_string(&task_id).unwrap()).await {
        Ok(result) => status::Custom(
            Status::Ok,
            json!({ "Success":true,"message":"view_one_todo is working fine","data":result }),
        ),
        Err(e) => status::Custom(
            Status::NotFound,
            json!({  "Success":false,"message":"view_one_todo is not working","data":e.to_string() }),
        ),
    }
}
#[options("/delete-one-todo/<_id>")]
pub fn delete_one_option(_id: String) -> status::Custom<Value> {
    println!("Going into options");
    status::Custom(Status::Ok, json! {"You are accessing home end point"})
}
