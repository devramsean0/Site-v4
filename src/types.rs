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
    pub fn new(value: Option<T>) -> Self {
        Self(value)
    }

    pub fn some(value: T) -> Self {
        Self(Some(value))
    }

    pub fn none() -> Self {
        Self(None)
    }

    pub fn as_option(&self) -> &Option<T> {
        &self.0
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

    // Add unwrap_or method
    pub fn unwrap_or(self, default: T) -> T {
        self.0.unwrap_or(default)
    }

    // Add unwrap_or_default method
    pub fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        self.0.unwrap_or_default()
    }

    // Add is_some and is_none methods
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }
    pub fn as_ref(&self) -> DisplayOption<&T> {
        DisplayOption(self.0.as_ref())
    }

    // Add map method
    pub fn map<U, F>(self, f: F) -> DisplayOption<U>
    where
        F: FnOnce(T) -> U,
    {
        DisplayOption(self.0.map(f))
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
