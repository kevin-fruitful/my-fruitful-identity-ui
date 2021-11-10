#[macro_use] extern crate rocket;

#[get("/")]
fn meta() -> &'static str {
    "Hello! The Fruitful Metaverse is under construction. Yes, I can't wait to play either"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![meta])
}