use uuid::Uuid;
use chrono::prelude::*;

pub struct EvtMeta {
    correlation: Uuid,
    occured_at: DateTime<Utc>,
}

impl EvtMeta {    
    pub fn new(correlation: Uuid, occured_at: DateTime<Utc>) -> Self {
        Self {
            correlation,
            occured_at, 
        }
    }

    pub fn new_now(correlation: Uuid) -> Self {
        Self {
            correlation,
            occured_at: Utc::now(), 
        }
    }

    pub fn correlation(&self) -> Uuid {
        self.correlation
    }

    pub fn occured_at(&self) -> DateTime<Utc> {
        self.occured_at
    }
}
