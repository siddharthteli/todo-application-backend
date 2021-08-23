use rocket::{
    catch, catchers,
    fairing::{Fairing, Info, Kind},
    get,
    http::{Header, Status},
    launch,
    response::status,
    routes,
    serde::json::{json, Value},
    Request, Response,
};

mod controller;
mod model;
mod service;
use controller::{create_one_todo, delete_one_todo, update_one_todo, view_all_todo, view_one_todo};

#[get("/home")]
fn home() -> status::Custom<Value> {
    status::Custom(Status::Ok, json! {"You are accessing home end point"})
}
#[catch(default)]
fn default(status: Status, req: &Request) -> String {
    format!("{} ({})", status, req.uri())
}
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                home,
                view_all_todo,
                view_one_todo,
                create_one_todo,
                update_one_todo,
                delete_one_todo,
            ],
        )
        .attach(CORS)
        .register("/", catchers![default])
}
