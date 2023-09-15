use diesel::result::Error as DieselError;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    JSONParsing(JSONParsingError),
    CLIParsing(ParsingCommandError),
    DatabaseError(DieselError),
}
#[derive(Debug)]
pub enum ParsingCommandError {
    TooFewArguments,
    CommandNotRecognized(String),
    TooManyArguments,
}
#[derive(Debug)]
pub enum JSONParsingError {
    KeyNotFound(String),
    // EmptyValue,
    UnexpectedTypeForKey(String),
}

// Implementations:

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<JSONParsingError> for Error {
    fn from(value: JSONParsingError) -> Self {
        Self::JSONParsing(value)
    }
}

impl From<ParsingCommandError> for Error {
    fn from(value: ParsingCommandError) -> Self {
        Self::CLIParsing(value)
    }
}

impl From<DieselError> for Error {
    fn from(value: DieselError) -> Self {
        Self::DatabaseError(value)
    }
}
