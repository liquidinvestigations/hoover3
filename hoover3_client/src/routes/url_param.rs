use std::{fmt::Display, str::FromStr};

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::Deref;

/// Helper for saving structs in URL as base64 strings.
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
    /// Create a new UrlParam with a value.
    pub fn new(value: T) -> Self {
        Self { value: Some(value) }
    }

    /// Check if the UrlParam is empty, as in it's never been initialized.
    pub fn is_empty(&self) -> bool {
        self.value.is_none()
    }

    /// Convert a signal of `UrlParam<T>` to a signal of `T` and a signal of bool
    /// indicating if the `UrlParam` has been initialized.
    ///
    /// This is required because of a bug/quirk in Dioxus where the value of the UrlParam signal
    /// is not available immediately after the component is mounted. The page with default T value
    /// will be loaded for a fraction of a second before the `UrlParam` is initialized from the URL.
    ///
    /// This short time is enough to initiate some backend calls that are immediately aborted, without
    /// us being able to log their cancellation, which causes the loading spinner to be shown continuously.
    ///
    /// The work-around implemented here is to return a second boolean signal that is used to prevent
    /// rendering the page until the `UrlParam` is initialized from the URL.
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
            write!(f, "{}", URL_SAFE_NO_PAD.encode(serialized))?;
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
        let decompressed = URL_SAFE_NO_PAD
            .decode(s.as_bytes())
            .map_err(StateParseError::DecodeError)?;
        let parsed = ciborium::from_reader(std::io::Cursor::new(decompressed))
            .map_err(StateParseError::CiboriumError)?;
        Ok(parsed)
    }
}
