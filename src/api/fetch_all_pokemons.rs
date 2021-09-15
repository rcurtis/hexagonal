use std::sync::Arc;
use crate::repositories::pokemon::Repository;
use crate::domain::fetch_all_pokemon;
use crate::api::Status;
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    number: u16,
    name: String,
    types: Vec<String>
}

pub fn serve(repo: Arc<dyn Repository>) -> rouille::Response {
    match fetch_all_pokemon::execute(repo) {
        Ok(res) => {
            let payload = &res.into_iter()
                .map(|p| Response {
                    number: p.number,
                    name: p.name,
                    types: p.types
                })
                .collect::<Vec<Response>>();
            rouille::Response::json(payload)
        },
        Err(fetch_all_pokemon::Error::Unknown) => {
            rouille::Response::from(Status::InternalServerError)
        }
    }
}