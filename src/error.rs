#[derive(Debug)]
pub enum Error {
    Serde(serde_json::Error),
    Io(std::io::Error),
    Other(String),
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::Serde(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::Other(value)
    }
}
