#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;

use model::clean_pokemon::CleanPokemon;
use model::error::Error;

use crate::client::poke_api_client::PokeApiClient;

mod model;
mod client;

static POKE_API_BASE_URL: &str = "https://pokeapi.co/";

#[get("/pokemon/<name>")]
async fn pokemon(name: &str) -> Result<Json<CleanPokemon>, Error> {
    let client = PokeApiClient::new(POKE_API_BASE_URL)?;

    let pokemon_species = client
        .get_pokemon_species(name)
        .await?;

    let clean_pokemon = CleanPokemon::new(pokemon_species);

    println!("clean_pokemon = {clean_pokemon:?}");

    Ok(Json(clean_pokemon))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![pokemon])
}
