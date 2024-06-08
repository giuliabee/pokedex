use rustemon::model::pokemon::PokemonSpecies;
use url::Url;

use crate::model::error::Error;
use crate::model::error::Error::{PokeApiDeserializationError, PokeApiError, PokemonNotFound};

pub struct PokeApiClient {
    url: Url,
}

impl PokeApiClient {
    pub fn new(url: &str) -> Result<Self, url::ParseError> {
        Ok(Self {
            url: Url::parse(url)?,
        })
    }

    pub async fn get_pokemon_species(self: &Self, name: &str) -> Result<PokemonSpecies, Error> {
        let url = self.url.join("api/v2/pokemon-species/")?.join(name)?;

        match reqwest::get(url).await {
            Ok(response) => match response.error_for_status() {
                Err(e) if e.status().is_some_and(|s| s == 404) => {
                    Err(PokemonNotFound("Pokemon not found"))
                }
                Err(_) => Err(PokeApiError("Error calling PokeAPI")),
                Ok(result) => match result.json::<PokemonSpecies>().await {
                    Ok(pokemon) => Ok(pokemon),
                    Err(_) => Err(PokeApiDeserializationError(
                        "Error deserializing PokeAPI response",
                    )),
                },
            },
            Err(_) => Err(PokeApiError("Error calling PokeAPI")),
        }
    }
}
