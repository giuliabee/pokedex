use url::ParseError;

#[derive(Responder)]
pub enum Error {
    #[response(status = 404)]
    NotFound(()),
    #[response(status = 500)]
    UnknownError(()),
    #[response(status = 500)]
    ParseError(()),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        match value.status() {
            Some(http_code) if http_code == 404 => Self::NotFound(()),
            Some(_) => Self::UnknownError(()),
            None => Self::UnknownError(()),
        }
    }
}

impl From<ParseError> for Error {
    fn from(_: ParseError) -> Self {
        Self::ParseError(())
    }
}
