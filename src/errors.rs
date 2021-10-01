use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Item larger than capacity")]
    ItemTooLargeError,
}
