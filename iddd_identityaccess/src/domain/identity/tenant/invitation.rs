use std::fmt;
use std::fmt::Formatter;

use chrono::prelude::*;

use iddd_common::{arg_error, assert_argument_length, assert_argument_not_empty, assert_true};
use iddd_common::assertion::ArgumentError;

/// Invitation structure.
#[derive(Debug)]
pub struct Invitation {
    invitation_id: String,
    description: String,
    starting_on: Option<DateTime<Utc>>,
    until: Option<DateTime<Utc>>,
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
            starting_on: None,
            until: None,
        })
    }

    /// Check if the [Invitation] is actually available.
    pub fn is_available(&self) -> bool {
        let now = Utc::now();
        return match self.starting_on {
            None => true,
            Some(start) => now >= start,
        } && match self.until {
            None => true,
            Some(end) => now <= end,
        };
    }

    /// Check if the [Invitation] is identified by the given identifier.
    ///
    /// # Arguments:
    /// * `identifier` - The invitation identifier.
    pub fn is_identified_by(&self, identifier: &str) -> bool {
        self.invitation_id == identifier || self.description == identifier
    }

    /// Redefine the [Invitation] as open-ended.
    pub fn redefine_as_open_ended(&mut self) {
        self.starting_on = None;
        self.until = None;
    }

    /// Redefine the [Invitation] with supplied validity range.
    ///
    /// # Arguments:
    /// * `starting_on` - The start of invitation validity.
    /// * `until` - The end of invitation validity.
    pub fn redefine_as(&mut self, starting_on: DateTime<Utc>, until: DateTime<Utc>) -> Result<(), ArgumentError> {
        assert_true!(starting_on <= until, "starting_on must occurs before until")?;
        self.starting_on = Some(starting_on);
        self.until = Some(until);
        Ok(())
    }
}

impl InvitationDescriptor for Invitation {
    fn invitation_id(&self) -> &str {
        &self.invitation_id
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn starting_on(&self) -> Option<DateTime<Utc>> {
        self.starting_on
    }

    fn until(&self) -> Option<DateTime<Utc>> {
        self.until
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
    fn starting_on(&self) -> Option<DateTime<Utc>>;

    fn until(&self) -> Option<DateTime<Utc>>;
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

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

    #[test]
    pub fn test_invitation_is_available_no_validity() {
        let actual = Invitation::new("anInvitationId", "aDescription");
        assert_eq!(true, actual.unwrap().is_available());
    }

    #[test]
    pub fn test_invitation_is_available_with_validity_in_past() {
        let past = Utc::now() - Duration::days(1);
        let mut actual = Invitation::new("anInvitationId", "aDescription")
            .unwrap();
        let _ = actual.redefine_as(DateTime::<Utc>::MIN_UTC, past);
        assert_eq!(false, actual.is_available());
    }

    #[test]
    pub fn test_invitation_is_available_with_validity_in_future() {
        let future = Utc::now() + Duration::days(1);
        let mut actual = Invitation::new("anInvitationId", "aDescription")
            .unwrap();
        let _ = actual.redefine_as(future, DateTime::<Utc>::MAX_UTC);
        assert_eq!(false, actual.is_available());
    }

    #[test]
    pub fn test_invitation_is_available_with_validity() {
        let past = Utc::now() - Duration::days(1);
        let future = Utc::now() + Duration::days(1);
        let mut actual = Invitation::new("anInvitationId", "aDescription")
            .unwrap();
        let _ = actual.redefine_as(past, future);
        assert_eq!(true, actual.is_available());
    }

    #[test]
    pub fn test_invitation_is_identified_by_invitation_id() {
        let invitation = Invitation::new("anInvitationId", "aDescription").unwrap();
        assert_eq!(true, invitation.is_identified_by("anInvitationId"));
    }

    #[test]
    pub fn test_invitation_is_identified_by_description() {
        let invitation = Invitation::new("anInvitationId", "aDescription").unwrap();
        assert_eq!(true, invitation.is_identified_by("aDescription"));
    }

    #[test]
    pub fn test_invitation_is_identified_not_identified() {
        let invitation = Invitation::new("anInvitationId", "aDescription").unwrap();
        assert_eq!(false, invitation.is_identified_by("missing"));
    }

    #[test]
    pub fn test_invitation_redefine_as_open_ended() {
        let mut invitation = Invitation::new("anInvitationId", "aDescription").unwrap();
        let _ = invitation.redefine_as(DateTime::<Utc>::MIN_UTC, DateTime::<Utc>::MAX_UTC);
        invitation.redefine_as_open_ended();
        assert_eq!(None, invitation.starting_on());
        assert_eq!(None, invitation.until());
    }

    #[test]
    pub fn test_invitation_redefine_as_invalid_end() {
        let mut invitation = Invitation::new("anInvitationId", "aDescription").unwrap();

        let res = invitation.redefine_as(DateTime::<Utc>::MAX_UTC, DateTime::<Utc>::MIN_UTC);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "starting_on must occurs before until")
    }
}