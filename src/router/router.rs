
pub mod user {
    use rocket::response::status;
    #[get("/user/<id>")]
    pub fn user(id: usize) -> status::Accepted<String> { 
        status::Accepted(Some(format!("user usize '{}'", id)))
    }

    #[get("/user/<id>", rank = 2)]
    pub fn user_int(id: isize) -> status::Accepted<String> { 
        status::Accepted(Some(format!("user isize '{}'", id)))
    }

    #[get("/user/<id>", rank = 3)]
    pub fn user_str(id: &str) -> status::Accepted<String> { 
        status::Accepted(Some(format!("user str '{}'", id)))
    }
}
