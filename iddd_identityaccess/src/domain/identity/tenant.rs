mod tenant_id;

use iddd_common::{arg_error, assert_argument_length, assert_argument_not_empty};
use iddd_common::assertion::ArgumentError;

pub use tenant_id::TenantId;

pub struct Tenant {
    tenant_id: TenantId,
    name: String,
    description: Option<String>,
    active: bool,
}

impl Tenant {
    pub fn new(tenant_id: TenantId, name: &str, description: Option<&str>, active: bool) -> Result<Tenant, ArgumentError> {
        assert_argument_not_empty!(name, "name")?;
        assert_argument_length!(name, 100, "name")?;
        if let Some(description) = description {
            assert_argument_length!(description, 100, "The description must be 100 characters or less.")?;
        }

        Ok(Tenant {
            tenant_id,
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            active,
        })
    }
}