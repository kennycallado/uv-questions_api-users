use serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ConfigGetter {
    pub profile_url: Option<String>,
    pub secret_key: Option<String>,
}

impl ConfigGetter {
    #[allow(unused)]
    pub fn get_profile_url() -> Option<String> {
        rocket::Config::figment()
            .extract::<ConfigGetter>()
            .unwrap()
            .profile_url
    }

    pub fn get_secret_key() -> Option<String> {
        rocket::Config::figment()
            .extract::<ConfigGetter>()
            .unwrap()
            .secret_key
        // .to_string()
    }
}
