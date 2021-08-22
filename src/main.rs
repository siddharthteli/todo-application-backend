use rocket::*;

#[get("/")]
fn world() -> &'static str {
    "Hello"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![world])
}
