use crate::domain::entities::{PokemonNumber, Pokemon};
use std::sync::Arc;
use crate::repositories::pokemon::{Repository, FetchOneError, DeleteError};
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

pub fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<(), Error> {
    let valid_num = PokemonNumber::try_from(req.number).map_err(|_| Error::BadRequest)?;

    match repo.delete(valid_num) {
        Ok(()) => Ok(()),
        Err(DeleteError::NotFound) => Err(Error::NotFound),
        Err(DeleteError::Unknown) => Err(Error::Unknown)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::repositories::pokemon::InMemoryRepository;
    use crate::domain::entities::{PokemonName, PokemonTypes};

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
    fn it_should_return_a_bad_request_when_request_is_invalid() {
        let repo = Arc::new(InMemoryRepository::new().with_error());
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
    fn it_should_return_ok_otherwise() {
        let repo = Arc::new(InMemoryRepository::new());
        repo.insert(PokemonNumber::pikachu(), PokemonName::pikachu(), PokemonTypes::pikachu()).ok();
        let req = Request::new(PokemonNumber::pikachu());

        let res = execute(repo, req);

        match res {
            Ok(()) => {},
            _ => unreachable!()
        }
    }
}