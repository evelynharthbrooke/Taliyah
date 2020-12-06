use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
    num::ParseIntError
};

use tracing::error;

#[derive(Debug)]
pub enum EllieError {
    DatabaseError(sqlx::Error),
    ParseError(ParseIntError),
    SerenityError(serenity::Error),
    CustomError(String)
}

impl Display for EllieError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut msg = String::from("[Error] ");
        let mut log_message = String::from(String::new());
        let error = match self {
            EllieError::DatabaseError(e) => e.to_string(),
            EllieError::ParseError(e) => e.to_string(),
            EllieError::SerenityError(e) => e.to_string(),
            EllieError::CustomError(e) => e.to_string()
        };
        msg += &error;
        log_message += &error;
        f.write_str(&msg)?;
        error!("Encountered an error: {}", &msg);
        Ok(())
    }
}

impl Error for EllieError {}

impl From<sqlx::Error> for EllieError {
    fn from(err: sqlx::Error) -> EllieError {
        EllieError::DatabaseError(err)
    }
}

impl From<String> for EllieError {
    fn from(err: String) -> EllieError {
        EllieError::CustomError(err)
    }
}

impl From<ParseIntError> for EllieError {
    fn from(err: ParseIntError) -> EllieError {
        EllieError::ParseError(err)
    }
}

impl From<serenity::Error> for EllieError {
    fn from(err: serenity::Error) -> EllieError {
        EllieError::SerenityError(err)
    }
}
