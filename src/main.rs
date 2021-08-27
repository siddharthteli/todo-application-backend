use rocket::{
    catch, catchers, delete,
    fairing::{Fairing, Info, Kind},
    get,
    http::{Header, Status},
    launch, options, post, put,
    response::status,
    routes,
    serde::json::{json, Value},
    Request, Response,
};

mod controller;
mod model;
mod service;
use controller::{
    create_one_todo, delete_one_option, delete_one_todo, test, test_option, update_one_option,
    update_one_todo, view_all_todo, view_one_todo,
};

//get-
#[get("/home")]
fn home() -> status::Custom<Value> {
    status::Custom(Status::Ok, json! {"You are accessing home end point"})
}
//post-
#[post("/home")]
fn home_post() -> status::Custom<Value> {
    status::Custom(Status::Ok, json! {"You are accessing home end point"})
}
//delete-
#[delete("/home/<_id>")]
fn home_delete(_id: i32) -> status::Custom<Value> {
    println!("Valueof someting:::-----{:?}", _id.clone());
    let message = json!({ "success": true, "message": "delete route", "data": _id});
    status::Custom(Status::Ok, message)
}
#[options("/home/<_id>")]
fn delete_options(_id: i32) -> status::Custom<Value> {
    println!("Going into options");
    status::Custom(Status::Ok, json! {"You are accessing home end point"})
}

//put-
#[put("/home")]
fn home_put() -> status::Custom<Value> {
    status::Custom(Status::Ok, json! {"You are accessing home end point"})
}
#[options("/home")]
fn put_options() -> status::Custom<Value> {
    println!("Going into options");
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
            "POST, GET, PATCH, OPTIONS, DELETE",
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
                test,
                home,
                home_put,
                test_option,
                home_post,
                put_options,
                home_delete,
                view_all_todo,
                view_one_todo,
                delete_options,
                delete_one_todo,
                create_one_todo,
                update_one_todo,
                update_one_option,
                delete_one_option,
            ],
        )
        .attach(CORS)
        .register("/", catchers![default])
}
