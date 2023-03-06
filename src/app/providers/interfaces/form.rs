use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Form {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub questions: Vec<Question>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Question {
    pub id: i32,
    pub q_type: String,
    pub question: String,
}
