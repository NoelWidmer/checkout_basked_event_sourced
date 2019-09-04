use uuid::Uuid;
use std::collections::HashMap;
use super::Item;

pub struct SnapshotData {
    id: Uuid, 
    items: HashMap<Uuid, Item>
}

impl SnapshotData {
    pub fn new(id: Uuid, items: HashMap<Uuid, Item>) -> Self {
        Self {
            id, 
            items
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn items(self) -> HashMap<Uuid, Item> {
        self.items
    }
}
