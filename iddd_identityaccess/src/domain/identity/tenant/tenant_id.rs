use uuid::Uuid;
use iddd_common::{arg_error, assert_argument_length, assert_argument_not_empty};
use iddd_common::assertion::ArgumentError;

/// The unique identifier for a tenant.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TenantId(String);

impl TenantId {
    /// Creates a new tenant identifier.
    pub fn new(value: &str) -> Result<Self, ArgumentError> {
        assert_argument_not_empty!(value, "value")?;
        assert_argument_length!(value, 36, 36, "value")?;
        if let Err(_) = Uuid::parse_str(value) {
            return arg_error!("The value as an invalid format.");
        }
        Ok(TenantId(value.to_string()))
    }

    /// Generates a new random tenant identifier.
    pub fn random() -> Self {
        TenantId(Uuid::new_v4().to_string())
    }

    /// Returns the value of tenant identifier.
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_new_empty_value() {
        let actual = TenantId::new("");
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err().to_string(), "The value is required");
    }

    #[test]
    pub fn test_new_short_value() {
        let actual = TenantId::new("a");
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err().to_string(), "The value must be 36 characters long");
    }

    #[test]
    pub fn test_new_long_value() {
        let actual = TenantId::new("12345678901234567890123456789012");
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err().to_string(), "The value must be 36 characters long");
    }

    #[test]
    pub fn test_new_invalid_value() {
        let actual = TenantId::new("12345678y1234r1234e1234p123456789012");
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err().to_string(), "The value as an invalid format.");
    }

    #[test]
    pub fn test_new_valid_value() {
        let actual = TenantId::new("12345678-1234-1234-1234-123456789012");
        assert!(actual.is_ok());
    }

    #[test]
    pub fn test_random() {
        let actual = TenantId::random();
        let parsed = TenantId::new(actual.value());
        assert!(parsed.is_ok());
        assert_eq!(actual.value(), parsed.unwrap().value());
    }
}