#[derive(Debug)]
pub enum Error {
  HeaderTooLong(usize),
  InvalidLocationFormat,
  InvalidHeaderEntryFormat,
  EmptyRequest,
  UnsupportedMethod(String),
  UnsupportedProtocol(String)
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        Error::HeaderTooLong(size) => write!(f, "Header is too long, exceeded size limit: {size}"),
        Error::InvalidLocationFormat => write!(f, "Invalid location format"),
        Error::InvalidHeaderEntryFormat => write!(f, "Invalid header entry format"),
        Error::EmptyRequest => write!(f, "Empty request"),
        Error::UnsupportedMethod(method) => write!(f, "Invalid request method: {method}"),
        Error::UnsupportedProtocol(protocol) => write!(f, "Invalid request protocol: {protocol}")
      }
  }
}

impl std::error::Error for Error {}