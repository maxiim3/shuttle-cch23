use rocket::{get, http, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
fn error() -> http::Status {
    http::Status::InternalServerError
}


#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![index ])
        .mount("/-1/error", routes![error]);

    Ok(rocket.into())
}
