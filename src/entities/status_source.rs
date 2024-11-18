use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct StatusSource {
    // ID of the status in the database
    pub id: String,
    // The plain text used to compose the status
    pub text: String,
    // The plain text used to compose the status’s subject or content warning
    pub spoiler_text: String,
}
