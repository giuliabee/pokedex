#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rustemon::model::pokemon::PokemonSpecies;

use model::clean_pokemon::CleanPokemon;
use model::error::Error;

mod model;

#[get("/pokemon/<name>")]
async fn pokemon(name: &str) -> Result<Json<CleanPokemon>, Error> {
    let pokemon_species = reqwest::get(format!(
        "https://pokeapi.co/api/v2/pokemon-species/{}",
        name
    ))
    .await?
    .error_for_status()?
    .json::<PokemonSpecies>()
    .await?;

    let clean_pokemon = CleanPokemon::new(pokemon_species);

    println!("clean_pokemon = {clean_pokemon:?}");

    Ok(Json(clean_pokemon))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, pokemon])
}
