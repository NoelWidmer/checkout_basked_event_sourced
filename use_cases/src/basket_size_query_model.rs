use std::sync::Arc;
use domain::{
    core::*,
    basket::*
};

pub struct BasketSizeQueryModel {
    store: Arc<EventStore<Basket>>
}

impl BasketSizeQueryModel {
    pub fn new(store: Arc<EventStore<Basket>>) -> Self {
        Self { store }
    }

    pub fn size(&self, basket_id: <Basket as IdTypeDef>::Id) -> Quantity {
        for evt in self.store.retrieve_all(&basket_id) {

        }

        Quantity::new(5).unwrap()
    }
}
