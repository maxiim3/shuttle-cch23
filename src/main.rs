use rocket::{get, http, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/error")]
fn error() -> http::Status {
    http::Status::InternalServerError
}


#[get("/<num1>/<num2>")]
fn xor(num1: u16, num2: u16) -> String {
    let xor = num1 ^ num2;
    let result: usize = xor.pow(3) as usize;
    format!("{}", result)
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![index ])
        .mount("/-1", routes![error])
        .mount("/1", routes![xor]);

    Ok(rocket.into())
}
