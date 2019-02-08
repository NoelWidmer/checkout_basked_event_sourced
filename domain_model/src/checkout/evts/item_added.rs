use crate::core::Entity;
use crate::checkout::Item;

pub struct ItemAdded {
    item: Entity<Item>
}

impl ItemAdded {
    pub fn new(item: Entity<Item>) -> Self {
        Self { item }
    }

    pub fn item(&self) -> &Entity<Item> {
        &self.item
    }
}
