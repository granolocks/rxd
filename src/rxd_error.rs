#[derive(Debug)]
#[non_exhaustive]
pub enum RxdError {
    CantOpenFile,
    CantReadFile,
    UnhandledReader,
}