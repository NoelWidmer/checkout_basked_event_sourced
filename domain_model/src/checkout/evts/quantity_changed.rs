use uuid::Uuid;
use crate::checkout::Quantity;

pub struct QuantityChanged {
    pub item_id: Uuid, 
    pub quantity: Quantity,
}