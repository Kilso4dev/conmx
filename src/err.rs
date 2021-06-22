use std::fmt;
use err_derive::Error;

#[derive(Debug, Clone, Error)]
pub enum ConmxErr {
    #[error(display = "Network Error: {}", _0)]
    Net(String),
    #[error(display = "UI Error: {}", _0)]
    Win(String),
    #[error(display = "Localization Error: {}", _0)]
    Locale(String),
}

