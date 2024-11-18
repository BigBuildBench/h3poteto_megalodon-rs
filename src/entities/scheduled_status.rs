use super::{Attachment, StatusParams};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ScheduledStatus {
    pub id: String,
    pub scheduled_at: DateTime<Utc>,
    pub params: StatusParams,
    pub media_attachments: Option<Vec<Attachment>>,
}
