mod otp;

use reqwest::Client;

use serde::Deserialize;

use std::time::Duration;

use ring::{
    hmac::{self, Key},
    rand::SystemRandom,
};

use super::DEFAULT_TIMEOUT_SECONDS;

#[derive(Deserialize)]
pub struct YubikeyConfig {
    /// Client ID for the Yubico API service
    client_id: u64,
    /// Secret key for the Yubico API service
    secret_key: String,
    /// The number of seconds until an external API request times out.
    /// If `None`, the `DEFAULT_TIMEOUT_SECONDS` will be used.
    api_timeout_seconds: Option<u64>,
}

pub struct Yubikey {
    config: YubikeyConfig,
    client: Client,
    key: Key,
    rng: SystemRandom,
}

#[derive(Debug)]
pub enum YubikeyError {
    RandError,
    NetworkError,
    NoStatus,
    NoData,
    BadData,
    NoSignature,
    BadSignature,
    NoSuchClient,
    OperationNotAllowed,
    MissingParameter,
    NotEnoughAnswers,
    NonceMismatch,
    SignatureMismatch,
    Other(String),
}

impl Yubikey {
    pub fn new(config: YubikeyConfig) -> Self {
        let timeout_seconds = config
            .api_timeout_seconds
            .unwrap_or(DEFAULT_TIMEOUT_SECONDS);
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout_seconds))
            .build()
            .unwrap();

        let key = Key::new(
            hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
            &base64::decode(&config.secret_key).unwrap(),
        );
        let rng = SystemRandom::new();

        Self {
            config,
            client,
            key,
            rng,
        }
    }
}
