extern crate core;

use rocket::{get, http, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/error")]
fn error() -> http::Status {
    http::Status::InternalServerError
}

#[get("/<num..>")]
fn xor(num: std::path::PathBuf) -> Result<String, http::Status> {
    let args = num.iter()
        .map(|arg| arg.to_str().unwrap().parse::<u32>())
        .collect::<Result<Vec<u32>, _>>();


    match args {
        Ok(args) => {
            if args.len() > 20 {
                Err(http::Status::BadRequest)
            } else {
                let result =
                    args.iter()
                        .fold(0, |acc, current| acc ^ current)
                        .pow(3) as usize;
                Ok(format!("{}", result))
            }
        },
        Err(_) => {
            Err(http::Status::NotAcceptable)
        }
    }
}


#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![index ])
        .mount("/-1", routes![error])
        .mount("/1", routes![xor]);

    Ok(rocket.into())
}
