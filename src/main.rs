use rocket::{
    catch, catchers, get,
    http::Status,
    launch, routes,
    serde::json::{json, Value},
    Request,
};

#[get("/home")]
fn home() -> Value {
    json! {"Hello"}
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
