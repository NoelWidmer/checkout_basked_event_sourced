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
    id: Uuid,
    items: HashMap<Uuid, Item>
}

impl IdTypeDef for Basket {
    type Id = Uuid;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl Basket {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Basket {
    fn default() -> Self { 
        Self {
            id: Uuid::new_v4(),
            items: HashMap::new()
        }
    }
}

impl Aggregate for Basket {
    type Kind = crate::AggregateKind;
    type SnapshotPayload = super::SnapshotData;
    type CmdPayload = super::CmdData;
    type EvtPayload = super::EvtData;
    type Error = super::Error;

    fn new_with_id(id: Uuid) -> Self {
        Self {
            id: id,
            items: HashMap::new()
        }
    }

    fn kind() -> crate::AggregateKind {
        crate::AggregateKind::Basket
    }
    
    fn try_from(data: Self::SnapshotPayload) -> Result<Self, super::Error> {
        let s = Self {
            id: data.id(), 
            items: data.items()
        };

        Ok(s)
    }

    fn into(&self) -> Self::SnapshotPayload {
        super::SnapshotData::new(self.id, self.items.clone())
    }

    fn handle(&self, cmd: &Cmd<Self>) -> Result<Vec<Evt<Self>>, super::Error> {
        let correlation = cmd.meta().correlation();

        match cmd.payload() {
            super::CmdData::AddItem(add_item) => self.add_item(correlation, add_item), 
            super::CmdData::RemoveItem(remove_item) => self.remove_item(correlation, remove_item),
            super::CmdData::ChangeQuantity(change_quantity) => self.change_quantity(correlation, change_quantity),
        }
    }

    fn apply(&mut self, evt: &Evt<Self>) -> Result<(), super::Error> {
        match evt.payload() {
            super::EvtData::ItemAdded(item_added) => self.item_added(item_added), 
            super::EvtData::ItemRemoved(item_removed) => self.item_removed(item_removed), 
            super::EvtData::QuantityChanged(quantity_changed) => self.quantity_changed(quantity_changed),
        }
    }
}

impl Basket {
    fn add_item(&self, correlation: Uuid, add_item: &AddItem) -> Result<Vec<Evt<Self>>, super::Error> {
        if self.items.iter().any(|(_, item)| item.product_id() == add_item.product_id) {
            Err(super::Error::ItemAlreadyPresent)
        } else {
            let item = {
                let item_id = Uuid::new_v4();
                let quantity = Quantity::new(1).expect("expected a quantity of 1");
                EntityProxy::new(Item::new(item_id, add_item.product_id, quantity))
            };

            let meta = EvtMeta::new_now(correlation);
            let subject = self.address();
            let evt_data = super::EvtData::ItemAdded(ItemAdded { item });
            Ok(vec![ Evt::new(meta, subject, evt_data) ])
        }
    }

    fn item_added(&mut self, item_added: &ItemAdded) -> Result<(), super::Error> {
        let existing_item = self.items.entry(*item_added.item.id());

        match existing_item {
            std::collections::hash_map::Entry::Occupied(_) => Err(super::Error::ItemAlreadyPresent), 
            std::collections::hash_map::Entry::Vacant(_) => {
                existing_item.or_insert(*item_added.item.entity());
                Ok(())
            }
        }
    }

    fn remove_item(&self, correlation: Uuid, remove_item: &RemoveItem) -> Result<Vec<Evt<Self>>, super::Error> {
        if self.items.contains_key(&remove_item.item_id) {
            let meta = EvtMeta::new_now(correlation);
            let subject = self.address();
            let evt_data = super::EvtData::ItemRemoved(ItemRemoved{ item_id: remove_item.item_id });
            Ok(vec![ Evt::new(meta, subject, evt_data) ])
        } else {
            Err(super::Error::ItemNotPresent)
        }
    }

    fn item_removed(&mut self, item_removed: &ItemRemoved) -> Result<(), super::Error> {
        self.items.remove(&item_removed.item_id);
        Ok(())
    }

    fn change_quantity(&self, correlation: Uuid, change_quantity: &ChangeQuantity) -> Result<Vec<Evt<Self>>, super::Error> {
        match self.items.get(&change_quantity.item_id) {
            Some(item) => {
                if item.quantity() == change_quantity.new_quantity {
                    // Nothing needs to change.
                    Ok(Vec::new())
                } else {
                    let meta = EvtMeta::new_now(correlation);
                    let subject = self.address();
                    let evt_data = super::EvtData::QuantityChanged(QuantityChanged{ 
                        item_id: change_quantity.item_id , 
                        new_quantity: change_quantity.new_quantity
                    });

                    Ok(vec![ Evt::new(meta, subject, evt_data) ])
                }
            },
            None => Err(super::Error::ItemNotPresent)
        }
    }

    fn quantity_changed(&mut self, quantity_changed: &QuantityChanged) -> Result<(), super::Error> {
        match self.items.get_mut(&quantity_changed.item_id) {
            Some(item) => {
                *item.quantity_mut() = quantity_changed.new_quantity;
                Ok(())
            }, 
            None => Err(super::Error::ItemNotPresent)
        }
    }
}
