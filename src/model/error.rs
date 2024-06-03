use url::ParseError;

#[derive(Responder)]
pub enum Error<'a> {
    #[response(status = 404)]
    NotFound(&'a str),
    #[response(status = 500)]
    UnknownError(String),
    #[response(status = 500)]
    ParseError(&'a str),
}

impl From<reqwest::Error> for Error<'_> {
    fn from(value: reqwest::Error) -> Self {
        match value.status() {
            Some(http_code) if http_code == 404 => Self::NotFound("Pokemon not found!"),
            Some(_) => Self::UnknownError(value.to_string()),
            None => Self::UnknownError("Unknown error".into()),
        }
    }
}

impl From<ParseError> for Error<'_> {
    fn from(_: ParseError) -> Self {
        Self::ParseError("Parsing error")
    }
}
