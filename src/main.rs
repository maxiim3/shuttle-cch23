extern crate core;

use std::path::PathBuf;
use std::str::FromStr;

use rocket::{get, http, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/error")]
fn error() -> http::Status {
    http::Status::InternalServerError
}

fn extract_args<T>(args: PathBuf) -> Result<Vec<T>, <T as FromStr>::Err>
    where
        T: FromStr,
        T::Err: ToString,
{
    args.iter()
        .map(|arg| arg.to_str().unwrap().parse::<T>())
        .collect::<Result<Vec<T>, _>>()
}


#[get("/<num..>")]
fn xor(num: PathBuf) -> Result<String, http::Status> {
    let args = extract_args::<u32>(num);

    match args {
        Ok(args) => {
            if args.len() > 20 && args.len() > 1 {
                Err(http::Status::BadRequest)
            } else {
                let result = args
                    .iter()
                    .fold(0, |acc, current| acc ^ current)
                    .pow(3) as usize;

                Ok(format!("{}", result))
            }
        }
        Err(_) => Err(http::Status::NotAcceptable),
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/-1", routes![error])
        .mount("/1", routes![xor]);

    Ok(rocket.into())
}
