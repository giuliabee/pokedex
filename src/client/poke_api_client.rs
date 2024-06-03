use rustemon::model::pokemon::PokemonSpecies;
use url::Url;

use crate::model::error::Error;

pub struct PokeApiClient {
    url: Url,
}

impl PokeApiClient {
    pub fn new(url: &str) -> Result<Self, url::ParseError> {
        Ok(Self {
            url: Url::parse(url)?,
        })
    }

    pub async fn get_pokemon_species(self: Self, name: &str) -> Result<PokemonSpecies, Error> {
        let url = self.url.join("api/v2/pokemon-species/")?.join(name)?;

        Ok(reqwest::get(url)
            .await?
            .error_for_status()?
            .json::<PokemonSpecies>()
            .await?)
    }
}
