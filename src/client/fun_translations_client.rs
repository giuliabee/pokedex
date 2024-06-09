use url::Url;

use crate::model::error::Error;
use crate::model::error::Error::{
    FunTranslationsApiDeserializationError, FunTranslationsApiError,
    FunTranslationsApiRateLimitError,
};
use crate::model::translation::TranslationResponse;

pub static FUN_TRANSLATIONS_BASE_URL: &str = "https://api.funtranslations.com/";

pub struct FunTranslationsClient {
    url: Url,
}

impl FunTranslationsClient {
    pub fn new(url: &str) -> Result<Self, url::ParseError> {
        Ok(Self {
            url: Url::parse(url)?,
        })
    }

    pub async fn translate<'a>(
        self: &Self,
        translation: &'a str,
        text: &'a str,
    ) -> Result<TranslationResponse, Error> {
        let mut url = self
            .url
            .join("translate/")?
            .join(format!("{translation}.json").as_str())?;

        url.query_pairs_mut().append_pair("text", text);

        match reqwest::get(url).await {
            Ok(response) => match response.error_for_status() {
                Err(e) if e.status().is_some_and(|s| s == 429) => Err(
                    FunTranslationsApiRateLimitError("Error Fun Translations API rate limit hit"),
                ),
                Err(_) => Err(FunTranslationsApiError(
                    "Error calling Fun Translations API",
                )),
                Ok(result) => match result.json::<TranslationResponse>().await {
                    Ok(translation) => Ok(translation),
                    Err(_) => Err(FunTranslationsApiDeserializationError(
                        "Error deserializing Fun Translations API response",
                    )),
                },
            },
            Err(_) => Err(FunTranslationsApiError(
                "Error calling Fun Translations API",
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use tokio;

    use crate::client::fun_translations_client::{
        FUN_TRANSLATIONS_BASE_URL, FunTranslationsClient,
    };

    #[tokio::test]
    #[ignore]
    // this test is ignored by default because it is subject to a 10 requests per hour rate limit
    async fn translate() {
        let client = FunTranslationsClient::new(FUN_TRANSLATIONS_BASE_URL)
            .expect("Could not instantiate Fun Translations API client");
        let text = "When several of these POKéMON gather, their electricity could build and cause lightning storms.";
        let response = client
            .translate("yoda", text)
            .await
            .expect("Fun Translations request failed");
        assert_eq!(response.get_translated_text().unwrap(), "When several of these pokémon gather,And cause lightning storms,  their electricity could build.")
    }
}
