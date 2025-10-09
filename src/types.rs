use rusqlite::types::{FromSql, FromSqlResult, ValueRef};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DisplayOption<T>(pub Option<T>);

impl<T: fmt::Display> fmt::Display for DisplayOption<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(val) => write!(f, "{}", val),
            None => Ok(()), // Display nothing for None
        }
    }
}

impl<T: FromSql> FromSql for DisplayOption<T> {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Null => Ok(DisplayOption::none()),
            _ => T::column_result(value).map(DisplayOption::some),
        }
    }
}

// Helper methods for convenience
impl<T> DisplayOption<T> {
    pub fn some(value: T) -> Self {
        Self(Some(value))
    }

    pub fn none() -> Self {
        Self(None)
    }

    pub fn into_option(self) -> Option<T> {
        self.0
    }
    pub fn unwrap(self) -> T
    where
        T: std::fmt::Debug,
    {
        self.0.unwrap()
    }

    // Add is_some and is_none methods
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }
}

impl DisplayOption<String> {
    pub fn as_deref(&self) -> DisplayOption<&str> {
        DisplayOption(self.0.as_deref())
    }
}

// Implement From traits for easy conversion
impl<T> From<Option<T>> for DisplayOption<T> {
    fn from(opt: Option<T>) -> Self {
        Self(opt)
    }
}

impl<T> From<T> for DisplayOption<T> {
    fn from(value: T) -> Self {
        Self(Some(value))
    }
}
