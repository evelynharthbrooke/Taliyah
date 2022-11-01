use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
    num::ParseIntError
};

use tracing::error;

#[derive(Debug)]
pub enum TaliyahError {
    Database(sqlx::Error),
    Parsing(ParseIntError),
    Serenity(serenity::Error),
    Custom(String)
}

impl Display for TaliyahError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let error = match self {
            TaliyahError::Database(e) => e.to_string(),
            TaliyahError::Parsing(e) => e.to_string(),
            TaliyahError::Serenity(e) => e.to_string(),
            TaliyahError::Custom(e) => e.to_string()
        };
        f.write_str(&error)?;
        error!("Encountered an error: {}", &error);
        Ok(())
    }
}

impl Error for TaliyahError {}

impl From<sqlx::Error> for TaliyahError {
    fn from(err: sqlx::Error) -> TaliyahError {
        TaliyahError::Database(err)
    }
}

impl From<String> for TaliyahError {
    fn from(err: String) -> TaliyahError {
        TaliyahError::Custom(err)
    }
}

impl From<ParseIntError> for TaliyahError {
    fn from(err: ParseIntError) -> TaliyahError {
        TaliyahError::Parsing(err)
    }
}

impl From<serenity::Error> for TaliyahError {
    fn from(err: serenity::Error) -> TaliyahError {
        TaliyahError::Serenity(err)
    }
}
