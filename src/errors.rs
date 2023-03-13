#[derive(Debug, PartialEq)]
pub enum ErrorCodes {
    FatalError,
    FoundFileInsteadOfDirectory,
    DirectoryNotFound,
}
