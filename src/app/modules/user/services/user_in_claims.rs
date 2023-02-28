use crate::app::{modules::user::model::UserExpanded, providers::interfaces::claims::{UserInClaims, RoleInClaims}};

impl From<UserExpanded> for UserInClaims {
    fn from(value: UserExpanded) -> Self {
        UserInClaims {
            id: value.id,
            depends_on: value.depends_on.depends_on,
            role: RoleInClaims {
                id: value.role.id,
                name: value.role.name,
            },
            user_token: value.user_token,
        }
    }
}
