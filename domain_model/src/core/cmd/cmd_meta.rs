use uuid::Uuid;
use chrono::prelude::*;

#[derive(Clone, Copy)]
pub struct CmdMeta {
    correlation: Uuid,
    issued_at: DateTime<Utc>,
}

impl CmdMeta {    
    pub fn new(correlation: Uuid, issued_at: DateTime<Utc>) -> Self {
        Self {
            correlation,
            issued_at, 
        }
    }

    pub fn new_now(correlation: Uuid) -> Self {
        Self {
            issued_at: Utc::now(), 
            correlation,
        }
    }

    pub fn correlation(&self) -> Uuid {
        self.correlation
    }

    pub fn issued_at(&self) -> DateTime<Utc> {
        self.issued_at
    }
}
