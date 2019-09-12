use crate::core::*;

pub struct Cmd<Agg: Aggregate> {
    meta: CmdMeta, 
    receiver: AggregateAddress<Agg>,
    payload: Agg::CmdPayload,
}

impl<Agg: Aggregate> Cmd<Agg> {
    pub fn new(meta: CmdMeta, receiver: AggregateAddress<Agg>, payload: Agg::CmdPayload) -> Self {
        Self {
            meta,
            receiver, 
            payload,
        }
    }

    pub fn meta(&self) -> &CmdMeta {
        &self.meta
    }

    pub fn receiver(&self) -> &AggregateAddress<Agg> {
        &self.receiver
    }

    pub fn payload(&self) -> &Agg::CmdPayload {
        &self.payload
    }
}
