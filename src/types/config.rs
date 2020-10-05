use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub endpoint: String,
}
