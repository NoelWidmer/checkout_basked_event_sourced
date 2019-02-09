use crate::core::Entity;
use crate::basket::Item;

pub struct ItemAdded {
    pub item: Entity<Item>
}
