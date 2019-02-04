use uuid::Uuid;
use chrono::prelude::*;

pub struct Command<Data> {
    correlation: Uuid,
    timestamp: DateTime<Utc>,
    data: Data
}

impl<Data> Command<Data> {
    pub fn new(correlation: Uuid, timestamp: DateTime<Utc>, data: Data) -> Self {
        Self {
            timestamp, 
            correlation, 
            data
        }
    }

    pub fn correlation(&self) -> Uuid {
        self.correlation
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    pub fn data(&self) -> &Data {
        &self.data
    }
}
