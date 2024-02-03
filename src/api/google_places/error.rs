use thiserror::Error;
use reqwest::Error as ReqwestError;
use std::env::VarError;


#[derive(Error, Debug)]
pub enum PlacesError {
    #[error("request  error: {0}")]
    Reqwest(#[from] ReqwestError),
    #[error("enviroment variable error: {0}")]
    EnvVar(#[from] VarError),
}