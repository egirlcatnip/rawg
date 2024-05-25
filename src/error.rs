use std::str::Utf8Error;
use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug)]
pub enum GetError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("serde_path_to_error error: {0}")]
    SerdePathToError(String),

    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] Utf8Error),

    #[error("Parse error")]
    ParseError(#[from] ParseError),

    #[error("RAWG API key is mandatory for use")]
    KeyNotProvided,

    #[error("Contacting the site resulted in an error")]
    SiteError,
}


/* #[derive(Error, Debug)]
pub enum ListError {
    #[error("No next page exists")]
    NoNextPage,
    #[error("No previous page exists")]
    NoPreviousPage,
    #[error("Page number {0} doesn't exist")]
    NoPage(i32),
}
 */
