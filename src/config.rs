use home::home_dir;
use rocket::serde::json::{serde_json, Value};
use serde::{Deserialize, Serialize};
use std::env::join_paths;
use std::fmt::{Debug, Display, Formatter};
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::Path;

const CHROMIUM_DEFAULT_PREFERENCES: &str = ".config/chromium/Default/Preferences";

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum Error {
    SerdeError(serde_json::Error),
    IoError(std::io::Error),
    JoinPathsError(std::env::JoinPathsError),
    HomeNotFound,
    NotAJsonObject(&'static str),
    KeyNotFound(&'static str),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SerdeError(error) => <serde_json::Error as Display>::fmt(error, f),
            Self::IoError(error) => <std::io::Error as Display>::fmt(error, f),
            Self::JoinPathsError(error) => <std::env::JoinPathsError as Display>::fmt(error, f),
            Self::HomeNotFound => write!(f, "home directory not found"),
            Self::NotAJsonObject(key) => write!(f, "not a JSON object: {key}"),
            Self::KeyNotFound(key) => write!(f, "JSON key not found: {key}"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::SerdeError(error)
    }
}

impl From<std::env::JoinPathsError> for Error {
    fn from(error: std::env::JoinPathsError) -> Self {
        Self::JoinPathsError(error)
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    url: String,
}

impl Config {
    #[must_use]
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    /// Applies the configuration to the system
    /// # Errors
    /// Returns an [`digsigctl::config::Error`] if the configuration could not be applied
    pub fn apply(&self) -> Result<(), Error> {
        let filename = join_paths([
            home_dir().ok_or(Error::HomeNotFound)?,
            CHROMIUM_DEFAULT_PREFERENCES.into(),
        ])?;
        let mut value = load(&filename)?;
        value
            .as_object_mut()
            .ok_or(Error::NotAJsonObject("preferences"))?
            .get_mut("session")
            .ok_or(Error::KeyNotFound("session"))?
            .as_object_mut()
            .ok_or(Error::NotAJsonObject("session"))?
            .insert("startup_urls".to_string(), vec![self.url.clone()].into());
        save(&filename, &value)
    }
}

fn load(filename: impl AsRef<Path>) -> Result<Value, Error> {
    Ok(serde_json::from_str::<Value>(&read_to_string(filename)?)?)
}

fn save(filename: impl AsRef<Path>, value: &Value) -> Result<(), Error> {
    Ok(OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename)?
        .write_all(serde_json::to_string(value)?.as_bytes())?)
}
