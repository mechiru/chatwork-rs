#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("query params serialization error: {0}")]
    QueryParams(#[from] serde_urlencoded::ser::Error),
    #[error("http client error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("http status code error: {0}")]
    StatusCode(reqwest::StatusCode),
}

pub type Result<T> = std::result::Result<T, Error>;
