mod router; // use router.rs

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![
        router::other::index,
        router::other::hello,
        router::other::test,
        router::user::user,
        router::user::user_int,
        router::user::user_str,
    ])
}