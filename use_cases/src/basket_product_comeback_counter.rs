use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;
use domain::{
    core::*,
    basket::*
};

pub struct BasketProductComebackCounter {
    store: Arc<EventStore<Basket>>
}

impl BasketProductComebackCounter {
    pub fn new(store: Arc<EventStore<Basket>>) -> Self {
        Self { store }
    }

    pub fn count(&self, basket_id: <Basket as IdTypeDef>::Id) -> Result<HashMap<Uuid, u32>, ()> {
        let mut product_id_2_count = HashMap::new();

        for evt in self.store.retrieve_all(&basket_id)? {
            let data = evt.data();

            match data {
                EvtData::ItemAdded(item_added) => {
                    let product_id = item_added.item.inner().product_id();

                    match product_id_2_count.get_mut(&product_id) {
                        Some(count) => *count += 1, 
                        None => { product_id_2_count.insert(product_id, 1); }
                    }
                }, 
                EvtData::ItemRemoved(_) => (), 
                EvtData::QuantityChanged(_) => (),
            }
        }

        Ok(product_id_2_count)
    }
}
