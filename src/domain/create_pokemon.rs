use crate::domain::entities::{PokemonNumber, PokemonName, PokemonTypes};
use std::convert::TryFrom;

struct Request {
    number: u16,
    name: String,
    types: Vec<String>,
}

enum Response {
    Ok(u16),
    BadRequest
}

fn execute(req: Request) -> Response {
    let poke_num = PokemonNumber::try_from(req.number);
    let poke_name = PokemonName::try_from(req.name);
    let poke_types = PokemonTypes::try_from(req.types);

    match (poke_num, poke_name, poke_types) {
        (Ok(number), Ok(_), Ok(_)) => Response::Ok(u16::from(number)),
        _ => Response::BadRequest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_the_pokemon_number_otherwise() {
        let number = 25;
        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")]
        };

        let res = execute(req);
        match res {
           Response::Ok(res_number) => { assert_eq!(res_number, number) }
            _ => unreachable!()
        }
    }
}