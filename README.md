# Pokédex API

This is a simple Pokédex application, it provides the following endpoints:

**Basic information**

`GET /pokemon/<pokemon_name>`

This endpoint returns basic information about a Pokémon retrieved from [PokéAPI](https://pokeapi.co/).

Example: `GET /pokemon/pikachu` returns:

```json
{
  "name": "pikachu",
  "description": "When several of these POKéMON gather, their electricity could build and cause lightning storms.",
  "habitat": "forest",
  "isLegendary": false
}
```

**Translated basic information**

`GET /pokemon/translated/<pokemon_name>`

This endpoint returns the same info as above but the description is translated
using [FunTranslations](https://funtranslations.com).

Example: `GET /pokemon/translated/pikachu` returns:

```json
{
  "name": "pikachu",
  "description": "At which hour several of these pokémon gather,  their electricity couldst buildeth and cause lightning storms.",
  "habitat": "forest",
  "isLegendary": false
}
```

## How to use with Docker

### How to build

Run `docker build` from the root directory of the project:

```
docker build -t pokedex . 
```

### How to run

```
docker run -it --rm -p 0.0.0.0:8000:8000 pokedex 
```

Add `-d` to run the container in background.

## How to use without Docker

You'll need a working Rust toolchain and Cargo. To build the project, just run

```
cargo build --release
```

The resulting binary is located in `target/release/pokedex`.

### How to run

```
export ROCKET_ADDRESS=0.0.0.0 
./target/release/pokedex
```

The `ROCKET_ADDRESS` environment variable configures the address on which the webserver will listen. Other configuration
parameters can be found [here](https://rocket.rs/guide/v0.5/configuration/#configuration).

## TODO for production/enterprise readiness

* caching to avoid repeated backend API requests
* implement FunTranslation API authentication since the free version is heavily rate limited (10 requests/hour)
* read API urls from configuration file
* improve error handling and reporting
* add `/status` endpoint for monitoring