use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RxdError {
    #[error("Can't open file")]
    CantOpenFile,

    #[error("Can't read file")]
    CantReadFile,

    #[error("Unhandled Reader Type")]
    UnhandledReader,

    #[error("Can't read STDIN")]
    CantReadStdIn,
}
