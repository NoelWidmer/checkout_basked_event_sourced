use uuid::Uuid;
use crate::core::*;
use super::cmds::*;
use super::evts::*;

pub struct CustomerOffer {

}

impl IdDefinition for CustomerOffer {
    type Id = Uuid;
}

impl AggregateRoot for CustomerOffer {
    type CmdData = super::CmdData;
    type HandleError = super::HandleError;
    type EvtData = super::EvtData;
    type ApplyError = super::ApplyError;

    fn handle(&self, cmd: &Cmd<super::CmdData>) -> Result<Vec<Evt<super::EvtData>>, super::HandleError> {
        let meta = cmd.meta();

        match cmd.data() {
            super::CmdData::AddItem(add_item) => self.add_item(meta, add_item), 
            super::CmdData::RemoveItem(remove_item) => self.remove_item(meta, remove_item)
        }
    }

    fn apply(&mut self, evt: &Evt<super::EvtData>) -> Result<(), super::ApplyError> {
        let meta = evt.meta();

        match evt.data() {
            super::EvtData::ItemAdded(item_added) => self.apply_item_added(meta, item_added), 
            super::EvtData::ItemRemoved(item_removed) => self.apply_item_removed(meta, item_removed)
        }
    }
}

impl CustomerOffer {
    fn add_item(&self, meta: &MsgMeta, add_item: &AddItem) -> Result<Vec<Evt<super::EvtData>>, super::HandleError> {
        Ok(Vec::new())
    }

    fn apply_item_added(&mut self, meta: &MsgMeta, item_added: &ItemAdded) -> Result<(), super::ApplyError> {
        Ok(())
    }

    fn remove_item(&self, meta: &MsgMeta, remove_item: &RemoveItem) -> Result<Vec<Evt<super::EvtData>>, super::HandleError> {
        Ok(Vec::new())
    }

    fn apply_item_removed(&mut self, meta: &MsgMeta, item_removed: &ItemRemoved) -> Result<(), super::ApplyError> {
        Ok(())
    }
}
