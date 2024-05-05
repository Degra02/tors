#[derive(Debug)]
pub enum Error {
    Inquire(inquire::InquireError),
    Serde(serde_json::Error),
    Io(std::io::Error),
    Other(String),

    #[cfg(feature = "clipboard")]
    WlCopy(arboard::Error),
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

impl From<inquire::InquireError> for Error {
    fn from(value: inquire::InquireError) -> Self {
        Error::Inquire(value)
    }
}

#[cfg(feature = "clipboard")]
impl From<arboard::Error> for Error {
    fn from(value: arboard::Error) -> Self {
        Error::WlCopy(value)
    }
}
