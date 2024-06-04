use url::Url;

use crate::model::error::Error;
use crate::model::translation::TranslationResponse;

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

        Ok(reqwest::get(url)
            .await?
            .error_for_status()?
            .json::<TranslationResponse>()
            .await?)
    }
}
