use iddd_common::{arg_error, assert_argument_length, assert_argument_not_empty};
use iddd_common::assertion::ArgumentError;
use crate::domain::identity::tenant::{Invitation, TenantId};

#[derive(Debug)]
pub struct Tenant {
    tenant_id: TenantId,
    name: String,
    description: Option<String>,
    active: bool,
    invitations: Vec<Invitation>,
}

impl Tenant {
    /// Creates a new [Tenant] with supplied parameters.
    ///
    /// # Arguments
    /// * `tenant_id` - The unique identifier of the tenant.
    /// * `name` - The name of the tenant.
    /// * `description` - The description of the tenant.
    /// * `active` - Indicates whether the tenant is active or not.
    pub fn new(tenant_id: &TenantId, name: &str, description: Option<&str>, active: bool) -> Result<Tenant, ArgumentError> {
        assert_argument_not_empty!(name, "name")?;
        assert_argument_length!(name, 100, "name")?;
        if let Some(description) = description {
            assert_argument_length!(description, 100, "description")?;
        }

        Ok(Tenant {
            tenant_id: tenant_id.clone(),
            name: name.to_string(),
            description: description.filter(|s| !s.is_empty()).map(|s| s.to_string()),
            active,
            invitations: Vec::new(),
        })
    }

    pub fn tenant_id(&self) -> &TenantId {
        return &self.tenant_id
    }

    pub fn name(&self) -> &str {
        return &self.name
    }

    pub fn description(&self) -> Option<&str> {
        return self.description.as_deref()
    }

    pub fn active(&self) -> bool {
        return self.active
    }

    /// Activates the tenant.
    pub fn activate(&mut self) {
        self.active = true;
    }

    /// Deactivates the tenant.
    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_new_empty_name() {
        let result = Tenant::new(&TenantId::random(), "", None, true);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "The name is required");
    }

    #[test]
    pub fn test_new_too_long_name() {
        let result = Tenant::new(&TenantId::random(), "0".repeat(101).as_str(), None, true);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "The name must be 100 characters or less");
    }

    #[test]
    pub fn test_new_empty_description() {
        let result = Tenant::new(&TenantId::random(), "name", Some(""), true);
        assert!(result.is_ok());
        assert_eq!(None, result.unwrap().description());
    }

    #[test]
    pub fn test_new_too_long_description() {
        let result = Tenant::new(&TenantId::random(), "name", Some("0".repeat(101).as_str()), true);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "The description must be 100 characters or less");
    }

    #[test]
    pub fn test_new_valid_tenant() {
        let tenant_id = TenantId::random();
        let result = Tenant::new(&tenant_id, "name", Some("description"), true);
        assert!(result.is_ok());
        let tenant = result.unwrap();
        assert_eq!(tenant_id.value(), tenant.tenant_id().value());
        assert_eq!("name", tenant.name());
        assert_eq!(Some("description"), tenant.description());
        assert_eq!(true, tenant.active());
    }

    #[test]
    pub fn test_activate() {
        let tenant_id = TenantId::random();
        let mut tenant = Tenant::new(&tenant_id, "name", Some("description"), false).unwrap();

        tenant.activate();
        assert_eq!(true, tenant.active());
    }

    #[test]
    pub fn test_deactivate() {
        let tenant_id = TenantId::random();
        let mut tenant = Tenant::new(&tenant_id, "name", Some("description"), true).unwrap();

        tenant.deactivate();
        assert_eq!(false, tenant.active());
    }
}