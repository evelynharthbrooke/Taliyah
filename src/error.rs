use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
    num::ParseIntError
};

use tracing::error;

#[derive(Debug)]
pub enum EllieError {
    Database(sqlx::Error),
    Parsing(ParseIntError),
    Serenity(serenity::Error),
    Custom(String)
}

impl Display for EllieError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let error = match self {
            EllieError::Database(e) => e.to_string(),
            EllieError::Parsing(e) => e.to_string(),
            EllieError::Serenity(e) => e.to_string(),
            EllieError::Custom(e) => e.to_string()
        };
        f.write_str(&error)?;
        error!("Encountered an error: {}", &error);
        Ok(())
    }
}

impl Error for EllieError {}

impl From<sqlx::Error> for EllieError {
    fn from(err: sqlx::Error) -> EllieError {
        EllieError::Database(err)
    }
}

impl From<String> for EllieError {
    fn from(err: String) -> EllieError {
        EllieError::Custom(err)
    }
}

impl From<ParseIntError> for EllieError {
    fn from(err: ParseIntError) -> EllieError {
        EllieError::Parsing(err)
    }
}

impl From<serenity::Error> for EllieError {
    fn from(err: serenity::Error) -> EllieError {
        EllieError::Serenity(err)
    }
}
