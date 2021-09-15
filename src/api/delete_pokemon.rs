use rouille;
use serde::Serialize;
use std::sync::Arc;
use crate::repositories::pokemon::Repository;
use crate::domain::entities::PokemonNumber;
use crate::api::Status;
use crate::domain::delete_pokemon::*;
use crate::domain::delete_pokemon;

pub fn serve(repo: Arc<dyn Repository>, number: u16) -> rouille::Response {
    let request = Request { number };
    match delete_pokemon::execute(repo, request) {
        Ok(_) => rouille::Response::from(Status::Ok),
        Err(Error::BadRequest) => rouille::Response::from(Status::BadRequest),
        Err(Error::NotFound) => rouille::Response::from(Status::NotFound),
        Err(Error::Unknown) => rouille::Response::from(Status::InternalServerError),
    }
}