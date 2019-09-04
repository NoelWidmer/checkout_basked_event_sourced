use uuid::Uuid;
use std::collections::HashMap;
use super::Item;

pub struct SnapshotData {
    items: HashMap<Uuid, Item>
}

impl SnapshotData {
    pub fn new(items: HashMap<Uuid, Item>) -> Self {
        Self {
            items
        }
    }

    pub fn items(self) -> HashMap<Uuid, Item> {
        self.items
    }
}
