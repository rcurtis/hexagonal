use crate::domain::entities::{PokemonNumber, PokemonName, PokemonTypes, Pokemon};
use crate::repositories::pokemon::{InMemoryRepository, Repository, InsertError};

use std::convert::TryFrom;
use std::sync::Arc;

pub struct Request {
    pub number: u16,
    pub name: String,
    pub types: Vec<String>,
}

pub struct Response {
    pub number: u16,
    pub name: String,
    pub types: Vec<String>,
}

pub enum Error {
    BadRequest,
    Conflict,
    Unknown,
}

pub fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<Response, Error> {
    let poke_num = PokemonNumber::try_from(req.number);
    let poke_name = PokemonName::try_from(req.name);
    let poke_types = PokemonTypes::try_from(req.types);

    match (poke_num, poke_name, poke_types) {
        (Ok(number), Ok(name), Ok(types)) =>  match repo.insert(number, name, types) {
            Ok(Pokemon {
                number,
                name,
                types
               }) => Ok(Response {
                number: u16::from(number),
                name: String::from(name),
                types: Vec::<String>::from(types)
            }),
            Err(InsertError::Conflict) => Err(Error::Conflict),
            Err(InsertError::Unknown) => Err(Error::Unknown)
        }
        _ => Err(Error::BadRequest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_the_pokemon_number_otherwise() {
        let repo = Arc::new(InMemoryRepository::new());
        let number = 25;
        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")]
        };

        let res = execute(repo, req);

        match res {
            Ok(Response { number, name, types }) => {
                assert_eq!(number, 25);
                assert_eq!(name, String::from("Pikachu"));
                assert_eq!(types, vec![String::from("Electric")])
            },
            _ => unreachable!()
        }
    }


    #[test]
    fn it_should_return_a_conflict_error_when_pokemon_number_already_exists() {
        let number = PokemonNumber::try_from(25).unwrap();
        let name = PokemonName::try_from(String::from("Pikachu")).unwrap();
        let types = PokemonTypes::try_from(vec![String::from("Electric")]).unwrap();
        let repo = Arc::new(InMemoryRepository::new());
        repo.insert(number, name, types);

        let req = Request {
            number: u16::from(number),
            name: String::from("Charmander"),
            types: vec![String::from("Fire")],
        };

        let res = execute(repo, req);

        match res {
            Err(Error::Conflict) => {},
            _ => unreachable!()
        }
    }

    #[test]
    fn it_should_return_an_unexpected_error_happens() {
        let mut repo = Arc::new(InMemoryRepository::new().with_error());
        let number = 25;
        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")]
        };

        let res = execute(repo, req);

        match res {
            Err(Error::Unknown) => {},
            _ => unreachable!()
        }
    }
}