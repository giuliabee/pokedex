#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::State;

use model::clean_pokemon::CleanPokemon;
use model::error::Error;

use crate::client::fun_translations_client::FunTranslationsClient;
use crate::client::poke_api_client::PokeApiClient;

mod client;
mod model;

static POKE_API_BASE_URL: &str = "https://pokeapi.co/";
static FUN_TRANSLATIONS_BASE_URL: &str = "https://api.funtranslations.com/";

struct Clients {
    poke_api_client: PokeApiClient,
    fun_translations_client: FunTranslationsClient,
}

#[get("/pokemon/<name>")]
async fn pokemon(name: &str, clients: &State<Clients>) -> Result<Json<CleanPokemon>, Error> {
    let pokemon_species = clients.poke_api_client.get_pokemon_species(name).await?;

    let clean_pokemon = CleanPokemon::new(pokemon_species);

    Ok(Json(clean_pokemon))
}

#[get("/pokemon/translated/<name>")]
async fn pokemon_translated(
    name: &str,
    clients: &State<Clients>,
) -> Result<Json<CleanPokemon>, Error> {
    let pokemon_species = clients.poke_api_client.get_pokemon_species(name).await?;

    let mut clean_pokemon = CleanPokemon::new(pokemon_species);

    let translation = if clean_pokemon.habitat == "cave" || clean_pokemon.is_legendary {
        "yoda"
    } else {
        "shakespeare"
    };

    let translated_description = clients
        .fun_translations_client
        .translate(translation, &clean_pokemon.description)
        .await?
        .get_translated_text()
        .unwrap_or(clean_pokemon.description);

    clean_pokemon.description = translated_description;

    Ok(Json(clean_pokemon))
}

#[launch]
fn rocket() -> _ {
    let poke_api_client =
        PokeApiClient::new(POKE_API_BASE_URL).expect("Could not instantiate PokéAPI client");
    let fun_translations_client = FunTranslationsClient::new(FUN_TRANSLATIONS_BASE_URL)
        .expect("Could not instantiate FunTranslations client");

    rocket::build()
        .mount("/", routes![pokemon, pokemon_translated])
        .manage(Clients {
            poke_api_client,
            fun_translations_client,
        })
}
