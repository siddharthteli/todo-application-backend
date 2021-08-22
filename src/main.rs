use rocket::{
    catch, catchers, get,
    http::Status,
    launch,
    response::status,
    routes,
    serde::json::{json, Value},
    Request,
};

mod controller;
mod model;
mod service;

#[get("/home")]
fn home() -> status::Custom<Value> {
    status::Custom(Status::Ok, json! {"You are accessing home end point"})
}
#[catch(default)]
fn default(status: Status, req: &Request) -> String {
    format!("{} ({})", status, req.uri())
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![home])
        .register("/", catchers![default])
}
