use uuid::Uuid;
use std::collections::HashMap;
use crate::core::*;
use super::{
    cmds::*, 
    evts::*, 
    Item, 
    Quantity
};

pub struct Basket {
    items: HashMap<Uuid, Item>
}

impl IdTypeDef for Basket {
    type Id = Uuid;
}

impl AggregateRoot for Basket {
    type CmdData = super::CmdData;
    type HandleError = super::HandleError;
    type EvtData = super::EvtData;
    type ApplyError = super::ApplyError;

    fn handle(&self, cmd: &Cmd<super::CmdData>) -> Result<Vec<Evt<super::EvtData>>, super::HandleError> {
        let correlation = cmd.meta().correlation();

        match cmd.data() {
            super::CmdData::AddItem(add_item) => self.add_item(correlation, add_item), 
            super::CmdData::RemoveItem(remove_item) => self.remove_item(correlation, remove_item),
            super::CmdData::ChangeQuantity(change_quantity) => self.change_quantity(correlation, change_quantity),
        }
    }

    fn apply(&mut self, evt: &Evt<super::EvtData>) -> Result<(), super::ApplyError> {
        match evt.data() {
            super::EvtData::ItemAdded(item_added) => self.item_added(item_added), 
            super::EvtData::ItemRemoved(item_removed) => self.item_removed(item_removed), 
            super::EvtData::QuantityChanged(quantity_changed) => self.quantity_changed(quantity_changed),
        }
    }
}

impl Basket {
    fn add_item(&self, correlation: Uuid, add_item: &AddItem) -> Result<Vec<Evt<super::EvtData>>, super::HandleError> {
        if self.items.iter().any(|(_, item)| item.product_id() == add_item.product_id) {
            Err(super::HandleError::ItemAlreadyAdded)
        } else {
            let item = {
                let item_id = Uuid::new_v4();
                let quantity = Quantity::new(1).expect("expected a quantity of 1");
                Entity::new(item_id, Item::new(add_item.product_id, quantity))
            };

            let meta = MsgMeta::new_now(correlation);
            let evt_data = super::EvtData::ItemAdded(ItemAdded { item });
            Ok(vec![ Evt::new(meta, evt_data) ])
        }
    }

    fn item_added(&mut self, item_added: &ItemAdded) -> Result<(), super::ApplyError> {
        let existing_item = self.items.entry(*item_added.item.id());

        match existing_item {
            std::collections::hash_map::Entry::Occupied(_) => Err(super::ApplyError::ItemAlreadyAdded), 
            std::collections::hash_map::Entry::Vacant(_) => {
                existing_item.or_insert(*item_added.item.inner());
                Ok(())
            }
        }
    }

    fn remove_item(&self, correlation: Uuid, remove_item: &RemoveItem) -> Result<Vec<Evt<super::EvtData>>, super::HandleError> {
        if self.items.contains_key(&remove_item.item_id) {
            let meta = MsgMeta::new_now(correlation);
            let evt_data = super::EvtData::ItemRemoved(ItemRemoved{ item_id: remove_item.item_id });
            Ok(vec![ Evt::new(meta, evt_data) ])
        } else {
            Err(super::HandleError::ItemNotYetAdded)
        }
    }

    fn item_removed(&mut self, item_removed: &ItemRemoved) -> Result<(), super::ApplyError> {
        self.items.remove(&item_removed.item_id);
        Ok(())
    }

    fn change_quantity(&self, correlation: Uuid, change_quantity: &ChangeQuantity) -> Result<Vec<Evt<super::EvtData>>, super::HandleError> {
        match self.items.get(&change_quantity.item_id) {
            Some(item) => {
                if item.quantity() == change_quantity.quantity {
                    Err(super::HandleError::QuantityRemainsUnchanged)
                } else {
                    let meta = MsgMeta::new_now(correlation);
                    let evt_data = super::EvtData::QuantityChanged(QuantityChanged{ 
                        item_id: change_quantity.item_id , 
                        quantity: change_quantity.quantity
                    });

                    Ok(vec![ Evt::new(meta, evt_data) ])
                }
            },
            None => Err(super::HandleError::ItemNotYetAdded)
        }
    }

    fn quantity_changed(&mut self, quantity_changed: &QuantityChanged) -> Result<(), super::ApplyError> {
        match self.items.get_mut(&quantity_changed.item_id) {
            Some(item) => {
                *item.quantity_mut() = quantity_changed.quantity;
                Ok(())
            }, 
            None => Err(super::ApplyError::ItemNotPresent)
        }
    }
}
