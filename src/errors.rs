use thiserror::Error;

#[derive(Error, Debug)]
pub enum NonaError {
    #[error("ERR_TEXTURE: {0}")]
    Texture(String),
}

pub type NonaResult<T> = Result<T, NonaError>;
