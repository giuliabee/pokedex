use rustemon::model::pokemon::PokemonSpecies;
use url::Url;

use crate::model::error::Error;
use crate::model::error::Error::{PokeApiDeserializationError, PokeApiError, PokemonNotFound};

pub static POKE_API_BASE_URL: &str = "https://pokeapi.co/";

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

#[cfg(test)]
mod test {
    use tokio;

    use crate::client::poke_api_client::{POKE_API_BASE_URL, PokeApiClient};
    use crate::model::clean_pokemon::CleanPokemon;
    use crate::model::error::Error::PokemonNotFound;

    #[tokio::test]
    async fn get_pokemon_species_existing() {
        let client =
            PokeApiClient::new(POKE_API_BASE_URL).expect("Could not instantiate PokéAPI client");
        let response = client
            .get_pokemon_species("pikachu")
            .await
            .expect("PokeAPI request failed");
        assert_eq!(
            CleanPokemon::new(response),
            CleanPokemon {
                name: "pikachu".to_string(),
                description: "When several of these POKéMON gather, their electricity could build and cause lightning storms.".to_string(),
                habitat: "forest".to_string(),
                is_legendary: false,
            }
        )
    }

    #[tokio::test]
    async fn get_pokemon_species_non_existing() {
        let client =
            PokeApiClient::new(POKE_API_BASE_URL).expect("Could not instantiate PokéAPI client");
        let error = client
            .get_pokemon_species("pika")
            .await
            .expect_err("PokeAPI request failed");
        assert_eq!(error, PokemonNotFound("Pokemon not found"))
    }
}
