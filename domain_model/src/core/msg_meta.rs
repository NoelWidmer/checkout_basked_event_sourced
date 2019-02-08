use uuid::Uuid;
use chrono::prelude::*;

#[derive(Clone, Copy)]
pub struct MsgMeta {
    correlation: Uuid,
    timestamp: DateTime<Utc>,
}

impl MsgMeta {    
    pub fn new(correlation: Uuid, timestamp: DateTime<Utc>) -> Self {
        Self {
            timestamp, 
            correlation,
        }
    }

    pub fn new_now(correlation: Uuid) -> Self {
        Self {
            timestamp: Utc::now(), 
            correlation,
        }
    }

    pub fn correlation(&self) -> Uuid {
        self.correlation
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}