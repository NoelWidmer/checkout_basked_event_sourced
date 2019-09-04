use crate::core::EntityProxy;
use crate::basket::Item;

pub struct ItemAdded {
    pub item: EntityProxy<Item>
}
