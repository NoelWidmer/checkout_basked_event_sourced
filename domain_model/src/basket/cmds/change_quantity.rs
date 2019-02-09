use uuid::Uuid;
use crate::basket::Quantity;

pub struct ChangeQuantity {
    pub item_id: Uuid, 
    pub quantity: Quantity,
}
