use super::*;
use uuid::Uuid;
use crate::core::IdTypeDef;

#[derive(Clone, Copy)]
pub struct Item {
    id: Uuid,
    product_id: Uuid, 
    quantity: Quantity,
}

impl IdTypeDef for Item {
    type Id = Uuid;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl Item {
    pub fn new(id: Uuid, product_id: Uuid, quantity: Quantity) -> Self {
        Self {
            id, 
            product_id, 
            quantity
        }
    }

    pub fn product_id(&self) -> Uuid {
        self.product_id
    }

    pub fn quantity(&self) -> Quantity {
        self.quantity
    }

    pub fn quantity_mut(&mut self) -> &mut Quantity {
        &mut self.quantity
    }
}
