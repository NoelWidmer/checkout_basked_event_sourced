use crate::core::*;

pub struct Evt<Agg: Aggregate> {
    meta: EvtMeta,
    subject: AggregateAddress<Agg>,
    payload: Agg::EvtPayload,
}

impl<Agg: Aggregate> Evt<Agg> {
    pub fn new(meta: EvtMeta, subject: AggregateAddress<Agg>, payload: Agg::EvtPayload) -> Self {
        Self {
            meta,
            subject,
            payload,
        }
    }

    pub fn meta(&self) -> &EvtMeta {
        &self.meta
    }

    pub fn subject(&self) -> &AggregateAddress<Agg> {
        &self.subject
    }

    pub fn payload(&self) -> &Agg::EvtPayload {
        &self.payload
    }
}
