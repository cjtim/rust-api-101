mod router;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![
        router::index,
        router::hello,
        router::user::user,
        router::user::user_int,
        router::user::user_str,
    ])
}