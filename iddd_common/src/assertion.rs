use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct ArgumentError(pub String);

impl Display for ArgumentError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ArgumentError {}

#[macro_export]
#[allow(unused_macros)]
macro_rules! arg_error {
    ($($arg:tt)*) => {
        Err(ArgumentError(format!($($arg)*)))
    }
}

#[macro_export]
macro_rules! assert_argument_equals {
    ($actual:expr, $expected:expr, $($arg:tt)*) => {{
        if $expected != $actual {
            return arg_error!("The {1} must be equal to {0}", $expected, $($arg)*);
        }
        Ok(())
    }}
}

#[macro_export]
macro_rules! assert_argument_false {
    ($actual:expr, $($arg:tt)*) => {{
        if $actual {
            return arg_error!("The {0} must be false", $($arg)*);
        }
        Ok(())
    }}
}

#[macro_export]
macro_rules! assert_argument_length {
    ($expected:expr, $minimum:expr, $maximum:expr, $($arg:tt)*) => {{
        if $expected.len() < $minimum || $expected.len() > $maximum {
            if $minimum == $maximum {
                return arg_error!("The {1} must be {0} characters long", $minimum, $($arg)*);
            }
            return arg_error!("The {2} must be long between {0} and {1} characters", $minimum, $maximum, $($arg)*);
        }
        Ok(())
    }};
    ($expected:expr, $maximum:expr, $($arg:tt)*) => {{
        if $expected.len() > $maximum {
            return arg_error!("The {1} must be {0} characters or less", $maximum, $($arg)*);
        }
        Ok(())
    }};
}

#[macro_export]
macro_rules! assert_argument_not_empty {
    ($actual:expr, $($arg:tt)*) => {{
        if $actual.is_empty() {
            return arg_error!("The {} is required", $($arg)*);
        }
        Ok(())
    }}
}

#[macro_export]
macro_rules! assert_argument_not_equals {
    ($actual:expr, $expected:expr, $($arg:tt)*) => {{
        if $expected == $actual {
            return arg_error!("The {1} must be different from {0}.", $actual, $($arg)*);
        }
        Ok(())
    }}
}

#[macro_export]
macro_rules! assert_argument_range {
    ($expected:expr, $minimum:expr, $maximum:expr, $($arg:tt)*) => {{
        if $expected < $minimum || $expected > $maximum {
            return arg_error!("The {2} must be between {0} and {1}.", $minimum, $maximum, $($arg)*);
        }
        Ok(())
    }}
}

#[macro_export]
macro_rules! assert_argument_true {
    ($actual:expr, $($arg:tt)*) => {{
        if !$actual {
            return arg_error!("The {0} must be true", $($arg)*);
        }
        Ok(())
    }}
}

#[macro_export]
macro_rules! assert_true {
    ($actual:expr, $($arg:tt)*) => {{
        if !$actual {
            return arg_error!($($arg)*);
        }
        Ok(())
    }}
}


#[derive(Debug)]
pub struct StateError(pub String);

impl Display for StateError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Error for StateError {}

#[macro_export]
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