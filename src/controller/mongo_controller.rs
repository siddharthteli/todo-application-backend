use rocket::{
    delete, get,
    http::Status,
    post, put,
    response::status,
    serde::json::{json, Value},
};

#[get("/view-one-todo")]
fn view_one_todo() -> status::Custom<Value> {
    status::Custom(Status::Ok, json!({"Success":true}))
}
