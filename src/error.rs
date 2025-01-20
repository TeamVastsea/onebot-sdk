#[derive(Debug)]
pub struct Error(pub ErrorKind);

#[derive(Debug)]
pub enum ErrorKind {
    ConnectError,
    ParseError(Option<serde_json::Error>),
    EventNotRecognised,
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error(ErrorKind::ParseError(Some(e)))
    }
}
