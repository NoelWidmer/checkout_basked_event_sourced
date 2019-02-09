use crate::core::Entity;
use crate::checkout::Item;

pub struct ItemAdded {
    pub item: Entity<Item>
}
