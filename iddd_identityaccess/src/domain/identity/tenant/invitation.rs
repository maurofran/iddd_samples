use std::fmt;
use std::fmt::Formatter;
use std::ops::{Bound, RangeBounds, RangeFull};

use chrono::prelude::*;

use iddd_common::{arg_error, assert_argument_length, assert_argument_not_empty};
use iddd_common::assertion::ArgumentError;

/// Invitation structure.
#[derive(Debug)]
pub struct Invitation<B> where B: RangeBounds<DateTime<Utc>> {
    invitation_id: String,
    description: String,
    validity: B,
}

impl Invitation<RangeFull> {
    /// Creates a new [Invitation].
    ///
    /// #Arguments
    /// * `invitation_id` - The invitation unique identifier.
    /// * `description` - The invitation description.
    pub fn new(invitation_id: &str, description: &str) -> Result<Invitation<RangeFull>, ArgumentError> {
        assert_argument_not_empty!(invitation_id, "invitation_id")?;
        assert_argument_length!(invitation_id, 1, 36, "invitation_id")?;
        assert_argument_not_empty!(description, "description")?;
        assert_argument_length!(description, 100, "description")?;
        Ok(Invitation {
            invitation_id: invitation_id.to_string(),
            description: description.to_string(),
            validity: RangeFull,
        })
    }
}

impl<B> Invitation<B> where B: RangeBounds<DateTime<Utc>> {
    /// Check if the [Invitation] is actually available.
    pub fn is_available(&self) -> bool {
        self.validity.contains(&Utc::now())
    }

    /// Check if the [Invitation] is identified by the given identifier.
    ///
    /// # Arguments:
    /// * `identifier` - The invitation identifier.
    pub fn is_identified_by(&self, identifier: &str) -> bool {
        self.invitation_id == identifier || self.description == identifier
    }

    /// Redefine the [Invitation] as open-ended.
    pub fn redefine_as_open_ended(self) -> Invitation<RangeFull> {
        Invitation {
            invitation_id: self.invitation_id,
            description: self.description,
            validity: RangeFull,
        }
    }

    /// Redefine the [Invitation] with supplied validity range.
    ///
    /// # Arguments:
    /// * `validity` - The validity range of the [Invitation].
    pub fn redefine_as<R>(self, validity: R) -> Invitation<R> where R: RangeBounds<DateTime<Utc>> {
        Invitation {
            invitation_id: self.invitation_id,
            description: self.description,
            validity,
        }
    }
}

impl<B> InvitationDescriptor for Invitation<B> where B: RangeBounds<DateTime<Utc>> {
    fn invitation_id(&self) -> &str {
        &self.invitation_id
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn starting_on(&self) -> Bound<&DateTime<Utc>> {
        self.validity.start_bound()
    }

    fn until(&self) -> Bound<&DateTime<Utc>> {
        self.validity.end_bound()
    }
}

impl<B> fmt::Display for Invitation<B> where B: RangeBounds<DateTime<Utc>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Invitation [invitation_id = {}, description = {}, starting_on = {:?}, until = {:?}]",
               self.invitation_id(), self.description(), self.starting_on(), self.until())
    }
}

pub trait InvitationDescriptor {
    fn invitation_id(&self) -> &str;
    fn description(&self) -> &str;
    fn starting_on(&self) -> Bound<&DateTime<Utc>>;

    fn until(&self) -> Bound<&DateTime<Utc>>;
}

#[cfg(test)]
mod tests {
    use std::ops::Bound::Unbounded;
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
        assert_eq!(invitation.starting_on(), Unbounded);
        assert_eq!(invitation.until(), Unbounded);
    }

    #[test]
    pub fn test_invitation_is_available_no_validity() {
        let actual = Invitation::new("anInvitationId", "aDescription");
        assert_eq!(true, actual.unwrap().is_available());
    }

    #[test]
    pub fn test_invitation_is_available_with_validity_in_past() {
        let past = Utc::now() - Duration::days(1);
        let actual = Invitation::new("anInvitationId", "aDescription")
            .unwrap().redefine_as(..past);
        assert_eq!(false, actual.is_available());
    }

    #[test]
    pub fn test_invitation_is_available_with_validity_in_future() {
        let future = Utc::now() + Duration::days(1);
        let actual = Invitation::new("anInvitationId", "aDescription")
           .unwrap().redefine_as(future..);
        assert_eq!(false, actual.is_available());
    }

    #[test]
    pub fn test_invitation_is_available_with_validity() {
        let past = Utc::now() - Duration::days(1);
        let future = Utc::now() + Duration::days(1);
        let actual = Invitation::new("anInvitationId", "aDescription")
          .unwrap().redefine_as(past..future);
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
}