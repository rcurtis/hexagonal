use rouille;
use serde::Serialize;
use std::sync::Arc;
use crate::repositories::pokemon::Repository;
use crate::domain::entities::PokemonNumber;
use crate::domain::fetch_pokemon;
use crate::domain::fetch_pokemon::Error;
use crate::api::Status;

#[derive(Serialize)]
struct Response {
    number: u16,
    name: String,
    types: Vec<String>
}

pub fn serve(repo: Arc<dyn Repository>, number: u16) -> rouille::Response {
    let req = fetch_pokemon::Request { number };
    match fetch_pokemon::execute(repo, req) {
        Ok(res) => {
            rouille::Response::json(&Response {
                number: res.number,
                name: res.name,
                types: res.types
            })
        },
        Err(fetch_pokemon::Error::BadRequest) => rouille::Response::from(Status::BadRequest),
        Err(fetch_pokemon::Error::NotFound) => rouille::Response::from(Status::NotFound),
        Err(fetch_pokemon::Error::Unknown) => rouille::Response::from(Status::InternalServerError),
    }
}