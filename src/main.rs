#[macro_use]
extern crate rocket;

use crate::client::fun_translations_client::FUN_TRANSLATIONS_BASE_URL;
use crate::client::poke_api_client::POKE_API_BASE_URL;

mod client;
mod model;
mod router;

#[launch]
pub fn rocket() -> _ {
    router::get_rocket_router(
        POKE_API_BASE_URL.to_string(),
        FUN_TRANSLATIONS_BASE_URL.to_string(),
    )
}
