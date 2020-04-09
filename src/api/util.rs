use reqwest::{Error as ReqwestError, Response, StatusCode};
use serde::de::DeserializeOwned;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub struct HtbError {
    pub message: String,
}

impl Display for HtbError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for HtbError {}

impl HtbError {
    pub fn new<S>(message: S) -> HtbError
    where
        S: Into<String>,
    {
        HtbError {
            message: message.into(),
        }
    }

    pub fn boxed<S>(message: S) -> Box<HtbError>
    where
        S: Into<String>,
    {
        box HtbError::new(message)
    }

    pub fn from(error: &dyn Error) -> HtbError {
        HtbError::new(error.to_string())
    }
}

pub trait HtbResponder {
    fn check(self) -> Result<Response, HtbError>;
}

impl HtbResponder for Result<Response, ReqwestError> {
    fn check(self) -> Result<Response, HtbError> {
        match self {
            Ok(response) => match response.status() {
                StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN | StatusCode::FOUND => Err(
                    HtbError::new("You are unauthorized, please check your API key"),
                ),
                StatusCode::OK => Ok(response),
                status => Err(HtbError::new(&format!(
                    "An unknown error occured (HTTP status {})",
                    status.as_u16(),
                ))),
            },
            Err(error) => match error.source() {
                Some(source) => Err(HtbError::from(source)),
                None => Err(HtbError::from(&error)),
            },
        }
    }
}

#[async_trait]
pub trait HtbParser {
    async fn from_json<T>(self) -> Result<T, HtbError>
    where
        T: DeserializeOwned;
}

#[async_trait]
impl HtbParser for Response {
    async fn from_json<T>(self) -> Result<T, HtbError>
    where
        T: DeserializeOwned,
    {
        match self.json().await {
            Ok(data) => Ok(data),
            Err(_) => Err(HtbError::new("Could not parse response from server")),
        }
    }
}
