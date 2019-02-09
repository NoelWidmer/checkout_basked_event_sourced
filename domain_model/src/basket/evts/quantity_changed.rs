use uuid::Uuid;
use crate::basket::Quantity;

pub struct QuantityChanged {
    pub item_id: Uuid, 
    pub new_quantity: Quantity,
}
