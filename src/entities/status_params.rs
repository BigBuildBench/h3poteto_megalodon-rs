use super::StatusVisibility;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct StatusParams {
    pub text: String,
    pub in_reply_to_id: Option<String>,
    pub media_ids: Option<Vec<String>>,
    pub sensitive: Option<bool>,
    pub spoiler_text: Option<String>,
    pub visibility: Option<StatusVisibility>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub application_id: Option<u32>,
}
