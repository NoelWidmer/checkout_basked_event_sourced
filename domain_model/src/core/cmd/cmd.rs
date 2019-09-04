use crate::core::*;

pub struct Cmd<Agg: Aggregate> {
    meta: CmdMeta, 
    receiver: AggregateAddress<Agg>,
    payload: Agg::CmdData,
}

impl<Agg: Aggregate> Cmd<Agg> {
    pub fn new(meta: CmdMeta, receiver: AggregateAddress<Agg>, payload: Agg::CmdData) -> Self {
        Self {
            meta,
            receiver, 
            payload,
        }
    }

    pub fn meta(&self) -> CmdMeta {
        self.meta
    }

    pub fn receiver(&self) -> &AggregateAddress<Agg> {
        &self.receiver
    }

    pub fn payload(&self) -> &Agg::CmdData {
        &self.payload
    }
}
