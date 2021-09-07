use rouille;
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    message: String
}

pub fn serve() -> rouille::Response {
    let response = Response { message: String::from("Gotta catch them all!") };
    rouille::Response::json(&response)
}