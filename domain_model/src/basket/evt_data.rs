use super::evts::*;

pub enum EvtData {
    ItemAdded(ItemAdded), 
    ItemRemoved(ItemRemoved), 
    QuantityChanged(QuantityChanged),
}