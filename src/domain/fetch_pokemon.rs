use crate::domain::entities::{PokemonNumber, Pokemon};
use std::sync::Arc;
use crate::repositories::pokemon::{Repository, FetchOneError};
use std::convert::TryFrom;

pub struct Request {
    pub number: u16,
}

#[derive(Debug)]
pub enum Error {
    Unknown,
    BadRequest,
    NotFound
}

#[derive(Debug)]
pub struct Response {
    pub number: u16,
    pub name: String,
    pub types: Vec<String>,
}

pub fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<Response, Error> {
    let valid_num = PokemonNumber::try_from(req.number).map_err(|e|Error::BadRequest)?;
    match repo.fetch_one(valid_num) {
        Ok(pokemon) => Ok(Response {
            number: pokemon.number.into(),
            name: pokemon.name.into(),
            types: pokemon.types.into()
        }),
        Err(FetchOneError::NotFound) => Err(Error::NotFound),
        Err(FetchOneError::Unknown) => Err(Error::Unknown)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::pokemon::{InMemoryRepository, Repository};
    use crate::domain::entities::{Pokemon, PokemonTypes, PokemonName};

    impl Request {
        fn new(number: PokemonNumber) -> Self { Self { number: u16::from(number) } }
    }

    #[test]
    fn it_should_return_an_unknown_error_when_an_unexpected_error_happens() {
        let repo = Arc::new(InMemoryRepository::new().with_error());
        let req = Request::new(PokemonNumber::pikachu());

        let res = execute(repo, req);

        match res {
            Err(Error::Unknown) => {},
            _ => unreachable!()
        }
    }

    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let repo = Arc::new(InMemoryRepository::new());
        let req = Request::new(PokemonNumber::bad());

        let res = execute(repo, req);

        match res {
            Err(Error::BadRequest) => {},
            _ => unreachable!()
        }
    }

    #[test]
    fn it_should_return_a_not_found_error_when_the_repo_doesnt_contain_the_pokemon() {
        let repo = Arc::new(InMemoryRepository::new());
        let req = Request::new(PokemonNumber::pikachu());

        let res = execute(repo, req);

        match res {
            Err(Error::NotFound) => {},
            _ => unreachable!()
        }
    }

    #[test]
    fn it_should_return_the_pokemon_otherwise() {
        let repo = Arc::new(InMemoryRepository::new());
        repo.insert(PokemonNumber::pikachu(), PokemonName::pikachu(), PokemonTypes::pikachu()).ok();
        let req = Request::new(PokemonNumber::pikachu());

        let res = execute(repo, req);
        dbg!(&res);

        match res {
            Ok(res) => {
                assert_eq!(res.number, u16::from(PokemonNumber::pikachu()));
                assert_eq!(res.name, String::from(PokemonName::pikachu()));
                assert_eq!(res.types, Vec::<String>::from(PokemonTypes::pikachu()))
            },
            _ => unreachable!()
        }
    }


}