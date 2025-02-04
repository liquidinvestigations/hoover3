use std::{fmt::Display, str::FromStr};

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::Deref;

// You can use a custom type with the hash segment as long as it implements Display, FromStr and Default
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UrlParam<T> {
    value: Option<T>,
}

impl<T> Deref for UrlParam<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Default + Clone> UrlParam<T> {
    pub fn new(value: T) -> Self {
        Self { value: Some(value) }
    }
    pub fn is_empty(&self) -> bool {
        self.value.is_none()
    }
    pub fn convert_signals(
        current_path: ReadOnlySignal<UrlParam<T>>,
    ) -> (ReadOnlySignal<T>, ReadOnlySignal<bool>) {
        // Memo does not work here, because the current_path signal comes from the router, and on refresh we're stuck with a default in our memo.
        // let path = use_memo(move || current_path.read().as_ref().cloned().unwrap_or_else(|| PathBuf::from(".")  ));
        let mut path = use_signal(move || T::default());
        let mut path_loaded = use_signal(|| false);
        use_effect(move || {
            let p = current_path.read().clone();
            if !p.is_empty() {
                path.set(p.as_ref().unwrap().clone());
                path_loaded.set(true);
            }
        });
        let path = ReadOnlySignal::new(path);
        let path_loaded = ReadOnlySignal::new(path_loaded);
        (path, path_loaded)
    }
}

impl<T: Default + Clone> From<T> for UrlParam<T> {
    fn from(value: T) -> Self {
        UrlParam::new(value)
    }
}

// Display the state in a way that can be parsed by FromStr
impl<T> Display for UrlParam<T>
where
    T: Serialize + for<'x> Deserialize<'x> + Clone + Debug + Default + PartialEq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut serialized = Vec::new();
        if ciborium::into_writer(self, &mut serialized).is_ok() {
            write!(f, "{}", STANDARD.encode(serialized))?;
        }
        Ok(())
    }
}

pub enum StateParseError {
    DecodeError(base64::DecodeError),
    CiboriumError(ciborium::de::Error<std::io::Error>),
}

impl std::fmt::Display for StateParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DecodeError(err) => write!(f, "Failed to decode base64: {}", err),
            Self::CiboriumError(err) => write!(f, "Failed to deserialize: {}", err),
        }
    }
}

// Parse the state from a string that was created by Display
impl<T> FromStr for UrlParam<T>
where
    T: Serialize + for<'x> Deserialize<'x> + Clone + Debug + Default + PartialEq,
{
    type Err = StateParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decompressed = STANDARD
            .decode(s.as_bytes())
            .map_err(StateParseError::DecodeError)?;
        let parsed = ciborium::from_reader(std::io::Cursor::new(decompressed))
            .map_err(StateParseError::CiboriumError)?;
        Ok(parsed)
    }
}
