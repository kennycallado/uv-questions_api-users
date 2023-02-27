use crate::database::schema::roles;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Queryable, Identifiable)]
#[serde(crate = "rocket::serde")]
pub struct Role {
    pub id: i32,
    pub name: String,
}
