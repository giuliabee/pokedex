use url::ParseError;

#[derive(Responder)]
pub enum Error {
    #[response(status = 404)]
    PokemonNotFound(&'static str),
    #[response(status = 500)]
    PokeApiError(&'static str),
    #[response(status = 500)]
    PokeApiDeserializationError(&'static str),

    #[response(status = 500)]
    FunTranslationsApiError(&'static str),
    #[response(status = 500)]
    FunTranslationsApiDeserializationError(&'static str),
    #[response(status = 429)]
    FunTranslationsApiRateLimitError(&'static str),

    #[response(status = 404)]
    NotFound(&'static str),
    #[response(status = 500)]
    UnknownError(&'static str),

    #[response(status = 500)]
    UrlParseError(&'static str),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        match value.status() {
            Some(http_code) if http_code == 404 => Self::NotFound("Error 404 not found"),
            Some(_) => Self::UnknownError("Unknown error occurred"),
            None => Self::UnknownError("Unknown error occurred"),
        }
    }
}

impl From<ParseError> for Error {
    fn from(_: ParseError) -> Self {
        Self::UrlParseError("Error parsing url")
    }
}
