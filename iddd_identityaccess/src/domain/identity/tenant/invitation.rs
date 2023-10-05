use std::fmt;
use std::fmt::Formatter;
use std::ops::Range;

use chrono::prelude::*;

use iddd_common::{arg_error, assert_argument_length, assert_argument_not_empty};
use iddd_common::assertion::ArgumentError;

/// Invitation structure.
#[derive(Debug)]
pub struct Invitation {
    invitation_id: String,
    description: String,
    validity: Option<Range<DateTime<Utc>>>,
}

impl Invitation {
    /// Creates a new [Invitation].
    ///
    /// #Arguments
    /// * `invitation_id` - The invitation unique identifier.
    /// * `description` - The invitation description.
    pub fn new(invitation_id: &str, description: &str) -> Result<Invitation, ArgumentError> {
        assert_argument_not_empty!(invitation_id, "invitation_id")?;
        assert_argument_length!(invitation_id, 1, 36, "invitation_id")?;
        assert_argument_not_empty!(description, "description")?;
        assert_argument_length!(description, 100, "description")?;
        Ok(Invitation {
            invitation_id: invitation_id.to_string(),
            description: description.to_string(),
            validity: None,
        })
    }

    /// Check if the [Invitation] is actually available.
    pub fn is_available(&self) -> bool {
        match self.validity {
            Some(ref range) => range.contains(&Utc::now()),
            None => true
        }
    }

    /// Check if the [Invitation] is identified by the given identifier.
    ///
    /// # Arguments:
    /// * `identifier` - The invitation identifier.
    pub fn is_identified_by(&self, identifier: &str) -> bool {
        self.invitation_id == identifier || self.description == identifier
    }

    /// Redefine the [Invitation] as open-ended.
    pub fn redefine_as_open_ended(&mut self) -> &mut Self {
        self.validity = None;
        self
    }

    /// Redefine the [Invitation] with supplied validity range.
    ///
    /// # Arguments:
    /// * `validity` - The validity range of the [Invitation].
    pub fn redefine_as(&mut self, validity: Range<DateTime<Utc>>) -> &mut Self {
        self.validity = Some(validity);
        self
    }
}

impl InvitationDescriptor for Invitation {
    fn invitation_id(&self) -> &str {
        &self.invitation_id
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn starting_on(&self) -> Option<&DateTime<Utc>> {
        self.validity.as_ref().map(|range| &range.start)
    }

    fn until(&self) -> Option<&DateTime<Utc>> {
        self.validity.as_ref().map(|range| &range.end)
    }
}

impl fmt::Display for Invitation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Invitation [invitation_id = {}, description = {}, starting_on = {:?}, until = {:?}]",
               self.invitation_id(), self.description(), self.starting_on(), self.until())
    }
}

pub trait InvitationDescriptor {
    fn invitation_id(&self) -> &str;
    fn description(&self) -> &str;
    fn starting_on(&self) -> Option<&DateTime<Utc>>;

    fn until(&self) -> Option<&DateTime<Utc>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_new_empty_invitation_id() {
        let actual = Invitation::new("", "aDescription");
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err().to_string(), "The invitation_id is required")
    }

    #[test]
    pub fn test_new_long_invitation_id() {
        let actual = Invitation::new("0".repeat(37).as_str(), "aDescription");
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err().to_string(), "The invitation_id must be long between 1 and 36 characters")
    }

    #[test]
    pub fn test_new_empty_invitation_description() {
        let actual = Invitation::new("anInvitationId", "");
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err().to_string(), "The description is required")
    }

    #[test]
    pub fn test_new_long_invitation_description() {
        let actual = Invitation::new("anInvitationId", "0".repeat(101).as_str());
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err().to_string(), "The description must be 100 characters or less")
    }

    #[test]
    pub fn test_new_invitation() {
        let actual = Invitation::new("anInvitationId", "aDescription");
        assert!(actual.is_ok());
        let invitation = actual.unwrap();
        assert_eq!(invitation.invitation_id(), "anInvitationId");
        assert_eq!(invitation.description(), "aDescription");
        assert_eq!(invitation.starting_on(), None);
        assert_eq!(invitation.until(), None);
    }
}