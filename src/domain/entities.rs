use std::convert::TryFrom;
use std::fmt::Error;

#[derive(Copy, Clone, PartialEq)]
pub struct PokemonNumber(u16);

impl TryFrom<u16> for PokemonNumber {
    type Error = ();

    fn try_from(n: u16) -> Result<Self, Self::Error> {
        if n > 0 && n < 899 {
            Ok(Self(n))
        } else {
            Err(())
        }
    }
}

impl From<PokemonNumber> for u16 {
    fn from(n: PokemonNumber) -> Self {
        n.0
    }
}

#[cfg(test)]
impl PokemonNumber {
    pub fn pikachu() -> Self { Self(25) }
}


#[derive(Clone)]
pub struct PokemonName(String);

impl TryFrom<String> for PokemonName {
    type Error = ();

    fn try_from(n: String) -> Result<Self, Self::Error> {
        if n.is_empty() {
            Err(())
        } else {
            Ok(Self(n))
        }
    }
}

impl From<PokemonName> for String {
    fn from(name: PokemonName) -> Self {
        name.0
    }
}

#[derive(Clone)]
pub struct PokemonTypes(Vec<PokemonType>);

impl TryFrom<Vec<String>> for PokemonTypes {
    type Error = ();

    fn try_from(ts: Vec<String>) -> Result<Self, Self::Error> {
        if ts.is_empty() {
            Err(())
        } else {
            let mut pts = vec![];
            for t in ts.iter() {
                match PokemonType::try_from(String::from(t)) {
                    Ok(pt) => pts.push(pt),
                    _ => return Err(())
                }
            }
            Ok(Self(pts))
        }
    }
}

impl From<PokemonTypes> for Vec<String> {
    fn from(pokemon_types: PokemonTypes) -> Self {
        let it = pokemon_types.0;
        it.into_iter()
            .map(|poke_type| { String::from(poke_type) })
            .collect()
    }
}

#[derive(Clone)]
enum PokemonType {
    Electric,
    Fire
}

impl TryFrom<String> for PokemonType {
    type Error = ();

    fn try_from(t: String) -> Result<Self, Self::Error> {
        match t.as_str() {
            "Electric" => Ok(Self::Electric),
            "Fire" => Ok(Self::Fire),
            _ => Err(())
        }
    }
}

impl From<PokemonType> for String {
    fn from(t: PokemonType) -> Self {
        match t {
            PokemonType::Electric => String::from("Electric"),
            PokemonType::Fire => String::from("Fire")
        }
    }
}

#[derive(Clone)]
pub struct Pokemon {
    pub number: PokemonNumber,
    pub name: PokemonName,
    pub types: PokemonTypes,
}

impl Pokemon {
    pub fn new(number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Self {
        Self { number, name, types }
    }
}