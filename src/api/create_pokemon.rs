use rouille;
use serde::{Deserialize, Serialize};
use crate::api::Status;
use crate::repositories::pokemon::Repository;
use std::sync::Arc;
use crate::domain::create_pokemon;

#[derive(Deserialize)]
struct Request {
    number: u16,
    name: String,
    types: Vec<String>
}

#[derive(Serialize)]
struct Response {
    number: u16,
    name: String,
    types: Vec<String>
}

pub fn serve(repo: Arc<dyn Repository>, req: &rouille::Request) -> rouille::Response {
    let req = match rouille::input::json::json_input::<Request>(req) {
        Ok(req) => create_pokemon::Request {
            number: req.number,
            name: req.name,
            types: req.types
        },
        _ => return rouille::Response::from(Status::BadRequest)
    };

    match create_pokemon::execute(repo, req) {
        Ok(response) => rouille::Response::json(&Response
        {
            number: response.number,
            name: response.name,
            types: response.types
        }),
        Err(create_pokemon::Error::BadRequest) => rouille::Response::from(Status::BadRequest),
        Err(create_pokemon::Error::Conflict) => rouille::Response::from(Status::Conflict),
        Err(create_pokemon::Error::Unknown) => rouille::Response::from(Status::InternalServerError)
    }
}