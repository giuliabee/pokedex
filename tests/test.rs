#[cfg(test)]
mod tests {
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use rocket::serde::json::serde_json;

    use pokedex::model::clean_pokemon::CleanPokemon;
    use pokedex::router::get_rocket_router;

    #[test]
    fn pokemon_mock() {
        let pikachu_json = include_str!("pikachu.json");

        let mut server = mockito::Server::new();

        server
            .mock("GET", "/api/v2/pokemon-species/pikachu")
            .with_status(201)
            .with_body(pikachu_json)
            .create()
            .expect(1);

        let router = get_rocket_router(server.url(), server.url());

        let client = Client::tracked(router).unwrap();

        let response = client.get("/pokemon/pikachu").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            serde_json::from_str::<CleanPokemon>(&response.into_string().unwrap()).unwrap(),
            CleanPokemon {
                name: "pikachu".to_string(),
                description: "When several of these POKéMON gather, their electricity could build and cause lightning storms.".to_string(),
                habitat: "forest".to_string(),
                is_legendary: false,
            }
        );
    }

    #[test]
    fn pokemon_does_not_exist() {
        let mut server = mockito::Server::new();

        server
            .mock("GET", "/api/v2/pokemon-species/pika")
            .with_status(404)
            .create()
            .expect(1);

        let router = get_rocket_router(server.url(), server.url());

        let client = Client::tracked(router).unwrap();

        let response = client.get("/pokemon/pika").dispatch();
        assert_eq!(response.status().code, 404);
    }

    #[test]
    fn pokemon_yoda_translation() {
        let mewtwo_json = include_str!("mewtwo.json");
        let mewtwo_translated_json = include_str!("mewtwo_translated.json");

        let mut poke_api_server = mockito::Server::new();
        let mut fun_translation_api_server = mockito::Server::new();

        poke_api_server
            .mock("GET", "/api/v2/pokemon-species/mewtwo")
            .with_status(201)
            .with_body(mewtwo_json)
            .create()
            .expect(1);

        fun_translation_api_server
            .mock("GET", "/translate/yoda.json")
            .match_query(mockito::Matcher::Regex("text=.*".to_string()))
            .with_status(200)
            .with_body(mewtwo_translated_json)
            .create()
            .expect(1);

        let router = get_rocket_router(poke_api_server.url(), fun_translation_api_server.url());

        let client = Client::tracked(router).unwrap();

        let response = client.get("/pokemon/translated/mewtwo").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            serde_json::from_str::<CleanPokemon>(&response.into_string().unwrap()).unwrap(),
            CleanPokemon {
                name: "mewtwo".to_string(),
                description: "Created by a scientist after years of horrific gene splicing and dna engineering experiments,  it was.".to_string(),
                habitat: "rare".to_string(),
                is_legendary: true,
            }
        );
    }

    #[test]
    fn pokemon_shakespeare_translation_e2e() {
        let pikachu_json = include_str!("pikachu.json");
        let pikachu_translated_json = include_str!("pikachu_translated.json");

        let mut poke_api_server = mockito::Server::new();
        let mut fun_translation_api_server = mockito::Server::new();

        poke_api_server
            .mock("GET", "/api/v2/pokemon-species/pikachu")
            .with_status(201)
            .with_body(pikachu_json)
            .create()
            .expect(1);

        fun_translation_api_server
            .mock("GET", "/translate/shakespeare.json")
            .match_query(mockito::Matcher::Regex("text=.*".to_string()))
            .with_status(200)
            .with_body(pikachu_translated_json)
            .create()
            .expect(1);

        let router = get_rocket_router(poke_api_server.url(), fun_translation_api_server.url());

        let client = Client::tracked(router).unwrap();

        let response = client.get("/pokemon/translated/pikachu").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            serde_json::from_str::<CleanPokemon>(&response.into_string().unwrap()).unwrap(),
            CleanPokemon {
                name: "pikachu".to_string(),
                description: "At which hour several of these pokémon gather,  their electricity couldst buildeth and cause lightning storms.".to_string(),
                habitat: "forest".to_string(),
                is_legendary: false,
            }
        );
    }
}
