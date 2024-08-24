use std::fmt;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum Error {
    /// Start of security.txt section could not be found.
    StartNotFound,
    /// End of security.txt section could not be found.
    EndNotFound,

    /// A field is not valid UTF-8.
    InvalidField { field: Vec<u8>, source: Utf8Error },
    /// A value is not valid UTF-8.
    InvalidValue { field: String, value: Vec<u8>, source: Utf8Error },
    /// A field with this name already exists.  
    DuplicateField { field: String },
    /// A field with an unknown name was found.
    UnknownField { field: String },
    /// A required field is missing.
    RequiredField { field: &'static str },
    /// A field was found, but no value exists.
    MissingValue { field: String },

    /// Contact is not in a valid format.
    InvalidContact { contact: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StartNotFound => write!(f, "failed to find start of security.txt section"),
            Self::EndNotFound => write!(f, "failed to find end of security.txt section"),
            Self::InvalidField { .. } => write!(f, "invalid utf-8 in field name"),
            Self::InvalidValue { field, .. } => write!(f, "invalid utf-8 in '{field}' field value"),
            Self::DuplicateField { field } => write!(f, "duplicate field '{field}'"),
            Self::UnknownField { field } => write!(f, "unknown field '{field}'"),
            Self::RequiredField { field } => write!(f, "required field '{field}' is missing"),
            Self::MissingValue { field } => write!(f, "field '{field}' is missing value"),
            Self::InvalidContact { contact } => write!(f, "invalid contact: {contact}"),
        }
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            Self::InvalidField { source, .. } => Some(source),
            Self::InvalidValue { source, .. } => Some(source),
            _ => None,
        }
    }
}
