use uuid::Uuid;

pub struct ItemRemoved {
    item_id: Uuid
}

impl ItemRemoved {
    pub fn new(item_id: Uuid) -> Self {
        Self { item_id }
    }

    pub fn item_id(&self) -> Uuid {
        self.item_id
    }
}
