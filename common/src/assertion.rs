use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct ArgumentError(String);

impl Display for ArgumentError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ArgumentError {}

#[allow(unused_macros)]
macro_rules! arg_error {
    ($($arg:tt)*) => {
        Err(ArgumentError(format!($($arg)*)))
    }
}

#[macro_export]
macro_rules! assert_argument_equals {
    ($expected:expr, $actual:expr, $($arg:tt)*) => {
        if $expected != $actual {
            arg_error!($($arg)*);
        }
    }
}

#[macro_export]
macro_rules! assert_argument_false {
    ($actual:expr, $($arg:tt)*) => {
        if $actual {
            arg_error!($($arg)*);
        }
    }
}

#[macro_export]
macro_rules! assert_argument_length {
    ($expected:expr, $maximum:expr, $($arg:tt)*) => {
        if $expected.len() > $maximum {
            arg_error!($($arg)*);
        }
    };
    ($expected:expr, $minimum:expr, $maximum:expr, $($arg:tt)*) => {
        if $expected.len() < $minimum || $expected.len() > $maximum {
            arg_error!($($arg)*);
        }
    }
}

#[macro_export]
macro_rules! assert_argument_not_empty {
    ($actual:expr, $($arg:tt)*) => {
        if $actual.is_empty() {
            arg_error!($($arg)*);
        }
    }
}

#[macro_export]
macro_rules! assert_argument_not_equals {
    ($expected:expr, $actual:expr, $($arg:tt)*) => {
        if $expected == $actual {
            arg_error!($($arg)*);
        }
    }
}

#[macro_export]
macro_rules! assert_argument_range {
    ($expected:expr, $minimum:expr, $maximum:expr, $($arg:tt)*) => {
        if $expected < $minimum || $expected > $maximum {
            arg_error!($($arg)*);
        }
    }
}

#[macro_export]
macro_rules! assert_argument_true {
    ($actual:expr, $($arg:tt)*) => {
        if !$actual {
            arg_error!($($arg)*);
        }
    }
}


#[derive(Debug)]
pub struct StateError(String);

impl Display for StateError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Error for StateError {}

#[allow(unused_macros)]
macro_rules! state_error {
    ($($arg:tt)*) => {
        Err(StateError(format!($($arg)*)))
    }
}

#[macro_export]
macro_rules! assert_not_state {
    ($actual:expr, $($arg:tt)*) => {
        if $actual {
            state_error!($($arg)*);
        }
    }
}

#[macro_export]
macro_rules! assert_state {
    ($actual:expr, $($arg:tt)*) => {
        if !$actual {
            state_error!($($arg)*);
        }
    }
}