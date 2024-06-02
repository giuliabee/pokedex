use regex::Regex;
use rustemon::model::pokemon::PokemonSpecies;

#[derive(Default, Debug, Clone, PartialEq, Eq, rocket::serde::Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CleanPokemon {
    pub name: String,
    pub description: String,
    pub habitat: String,
    pub is_legendary: bool,
}

impl CleanPokemon {
    pub fn new(pokemon_species: PokemonSpecies) -> Self {
        let regex = Regex::new(r"[\u0000-\u001F]").unwrap();

        // get any English description or empty string if none is found
        let description = match pokemon_species
            .flavor_text_entries
            .iter()
            .find(|&p| p.language.name == "en")
        {
            Some(flavor_text) => flavor_text.flavor_text.clone(),
            None => "".into(),
        };

        // replace non printable characters
        let clean_description = regex.replace_all(&*description, " ").to_string();

        // get habitat name or empty string
        let clean_habitat = match pokemon_species.habitat {
            Some(habitat) => habitat.name,
            None => "".into(),
        };

        Self {
            name: pokemon_species.name,
            description: clean_description,
            habitat: clean_habitat,
            is_legendary: pokemon_species.is_legendary,
        }
    }
}
