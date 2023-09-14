#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    JSONParsing(JSONParsingError),
    CLIParsing(ParsingCommandError),
}
// TODO: Make these a single kind of error.
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
