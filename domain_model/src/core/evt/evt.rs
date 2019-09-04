use crate::core::*;

pub struct Evt<Payload> {
    meta: EvtMeta,
    payload: Payload,
}

impl<Payload> Evt<Payload> {
    pub fn new(meta: EvtMeta, payload: Payload) -> Self {
        Self {
            meta,
            payload,
        }
    }

    pub fn meta(&self) -> EvtMeta {
        self.meta
    }

    pub fn payload(&self) -> &Payload {
        &self.payload
    }
}

impl<Payload: Copy> Copy for Evt<Payload> { }

impl<Payload: Clone> Clone for Evt<Payload> {
    fn clone(&self) -> Self {
        Self::new(self.meta, self.payload.clone())
    }
}
