#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    rocket::serde::Serialize,
    rocket::serde::Deserialize
)]
#[serde(crate = "rocket::serde")]
pub struct TranslationResponse {
    success: Option<Total>,
    error: Option<TranslationError>,
    contents: Translation,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    rocket::serde::Serialize,
    rocket::serde::Deserialize
)]
#[serde(crate = "rocket::serde")]
struct TranslationError {
    code: u64,
    message: String,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    rocket::serde::Serialize,
    rocket::serde::Deserialize
)]
#[serde(crate = "rocket::serde")]
struct Total {
    total: u64,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    rocket::serde::Serialize,
    rocket::serde::Deserialize
)]
#[serde(crate = "rocket::serde")]
struct Translation {
    translated: String,
    text: String,
    translation: String,
}

impl TranslationResponse {
    pub fn get_translated_text(self: Self) -> Option<String> {
        match self.success {
            Some(_) => Some(self.contents.translated),
            None => None,
        }
    }
}
