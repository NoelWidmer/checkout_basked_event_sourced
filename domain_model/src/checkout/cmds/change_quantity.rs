use uuid::Uuid;
use crate::checkout::Quantity;

pub struct ChangeQuantity {
    pub item_id: Uuid, 
    pub quantity: Quantity,
}
