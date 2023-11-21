#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid Node URL scheme {0}")]
    NodeURLScheme(String),

    #[error("Unable to parse Node URL {0}")]
    NodeURLParser(#[from] url::ParseError),

    #[error(transparent)]
    IndexerError(#[from] chainthru_index::Error),

    #[error(transparent)]
    ServerError(#[from] chainthru_server::Error),
}
