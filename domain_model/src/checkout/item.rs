use super::*;
use uuid::Uuid;
use crate::core::IdTypeDef;

#[derive(Clone, Copy)]
pub struct Item {
    product_id: Uuid, 
    quantity: Quantity,
}

impl IdTypeDef for Item {
    type Id = Uuid;
}

impl Item {
    pub fn new(product_id: Uuid, quantity: Quantity) -> Self {
        Self {
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
