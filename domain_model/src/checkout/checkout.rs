use uuid::Uuid;
use std::collections::HashMap;
use crate::core::*;
use super::{
    cmds::*, 
    evts::*, 
    Item, 
    Quantity
};

pub struct Checkout {
    items: HashMap<Uuid, Item>
}

impl IdTypeDef for Checkout {
    type Id = Uuid;
}

impl AggregateRoot for Checkout {
    type CmdData = super::CmdData;
    type HandleError = super::HandleError;
    type EvtData = super::EvtData;
    type ApplyError = super::ApplyError;

    fn handle(&self, cmd: &Cmd<super::CmdData>) -> Result<Vec<Evt<super::EvtData>>, super::HandleError> {
        let correlation = cmd.meta().correlation();

        match cmd.data() {
            super::CmdData::AddItem(add_item) => self.add_item(correlation, add_item), 
            super::CmdData::RemoveItem(remove_item) => self.remove_item(correlation, remove_item)
        }
    }

    fn apply(&mut self, evt: &Evt<super::EvtData>) -> Result<(), super::ApplyError> {
        match evt.data() {
            super::EvtData::ItemAdded(item_added) => self.apply_item_added(item_added), 
            super::EvtData::ItemRemoved(item_removed) => self.apply_item_removed(item_removed)
        }
    }
}

impl Checkout {
    fn add_item(&self, correlation: Uuid, add_item: &AddItem) -> Result<Vec<Evt<super::EvtData>>, super::HandleError> {
        if self.items.iter().any(|(_, item)| item.product_id() == add_item.product_id()) {
            Err(super::HandleError::ItemAlreadyAdded)
        } else {
            let item = {
                let item_id = Uuid::new_v4();
                let quantity = Quantity::new(1).expect("expected a quantity of 1");
                Entity::new(item_id, Item::new(add_item.product_id(), quantity))
            };

            let meta = MsgMeta::new_now(correlation);
            let evt_data = super::EvtData::ItemAdded(ItemAdded::new(item));
            Ok(vec![ Evt::new(meta, evt_data) ])
        }
    }

    fn apply_item_added(&mut self, item_added: &ItemAdded) -> Result<(), super::ApplyError> {
        let existing_item = self.items.entry(*item_added.item().id());

        match existing_item {
            std::collections::hash_map::Entry::Occupied(_) => Err(super::ApplyError::ItemAlreadyAdded), 
            std::collections::hash_map::Entry::Vacant(_) => {
                existing_item.or_insert(*item_added.item().inner());
                Ok(())
            }
        }
    }

    fn remove_item(&self, correlation: Uuid, remove_item: &RemoveItem) -> Result<Vec<Evt<super::EvtData>>, super::HandleError> {
        let item_id_to_remove = remove_item.item_id();

        if self.items.contains_key(&item_id_to_remove) {
            let meta = MsgMeta::new_now(correlation);
            let evt_data = super::EvtData::ItemRemoved(ItemRemoved::new(item_id_to_remove));
            Ok(vec![ Evt::new(meta, evt_data) ])
        } else {
            Err(super::HandleError::ItemNotYetAdded)
        }
    }

    fn apply_item_removed(&mut self, item_removed: &ItemRemoved) -> Result<(), super::ApplyError> {
        match self.items.remove(&item_removed.item_id()) {
            Some(_) => Ok(()),
            None => Err(super::ApplyError::ItemNotPresent)
        }
    }
}
