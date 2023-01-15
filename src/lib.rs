use std::env;
use serde::Deserialize;

pub mod models;
pub mod embeddings;

pub(crate) const BASE_URL: &str = "https://api.openai.com/v1";

pub(crate) fn get_token() -> String {
    env::var("OPENAI_KEY")
        .expect("environment variable `OPENAI_KEY` should be defined")
}

#[derive(Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub total_tokens: u64,
}
