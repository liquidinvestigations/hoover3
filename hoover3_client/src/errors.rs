// use anyhow;
pub trait AnyhowErrorDioxusExt {
    type ReturnType;
    fn throw(self) -> Result<Self::ReturnType, dioxus::prelude::RenderError>;
    fn throw_context(self, context: &str)
        -> Result<Self::ReturnType, dioxus::prelude::RenderError>;
}

// impl<T> AnyhowErrorDioxusExt for anyhow::Result<T> {
//     type ReturnType = T;
//     fn throw(self) -> Result<Self::ReturnType, dioxus::prelude::RenderError> {
//         self.throw_context("UI Component Error")
//     }
//     fn throw_context(self, context: &str) -> Result<T, dioxus::prelude::RenderError> {
//         self.map_err(|e| {
//             dioxus::prelude::RenderError::Aborted(dioxus::CapturedError::from_display(format!(
//                 "{context}: {e}"
//             )))
//         })
//     }
// }

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
