use diesel::result::Error as DieselError;
// use thiserror::Error;
// todo: implement trait thiserror::Error

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    CLIParsing(ParsingCommandError),
    DatabaseError(DieselError),
    InvalidInput(String),
}
#[derive(Debug)]
pub enum ParsingCommandError {
    TooFewArguments,
    CommandNotRecognized(String),
    TooManyArguments,
    QuizNotFound(String),
}

// Implementations:

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
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
