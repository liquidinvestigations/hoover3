//! This module contains the error handling for the Hoover3 client.
//! It provides an implementation for the `.throw()` method for Results and Options,

/// Trait for the `.throw()` method for Results and Options. Used like in Dioxus 0.5, see online docs
pub trait AnyhowErrorDioxusExt {
    /// The type of the value that is being returned.
    type ReturnType;
    /// Throw the error, with a default context string.
    fn throw(self) -> Result<Self::ReturnType, dioxus::prelude::RenderError>;
    /// Throw the error, with a custom context string.
    fn throw_context(self, context: &str)
        -> Result<Self::ReturnType, dioxus::prelude::RenderError>;
}

impl<T, E> AnyhowErrorDioxusExt for Result<T, E>
where
    E: std::fmt::Debug,
{
    type ReturnType = T;
    fn throw(self) -> Result<Self::ReturnType, dioxus::prelude::RenderError> {
        self.throw_context("UI Component Error")
    }
    fn throw_context(self, context: &str) -> Result<T, dioxus::prelude::RenderError> {
        self.map_err(|e| {
            dioxus::prelude::RenderError::Aborted(dioxus::CapturedError::from_display(format!(
                "{context}: {e:#?}"
            )))
        })
    }
}

impl<T> AnyhowErrorDioxusExt for Option<T> {
    type ReturnType = T;
    fn throw(self) -> Result<Self::ReturnType, dioxus::prelude::RenderError> {
        self.throw_context("UI Component Error")
    }
    fn throw_context(self, context: &str) -> Result<T, dioxus::prelude::RenderError> {
        self.ok_or_else(|| {
            dioxus::prelude::RenderError::Aborted(dioxus::CapturedError::from_display(format!(
                "{context}: throw() called on None."
            )))
        })
    }
}
