use crate::core::*;

pub struct Cmd<Payload> {
    meta: CmdMeta, 
    payload: Payload,
}

impl<Payload> Cmd<Payload> {
    pub fn new(meta: CmdMeta, payload: Payload) -> Self {
        Self {
            meta,
            payload,
        }
    }

    pub fn meta(&self) -> CmdMeta {
        self.meta
    }

    pub fn payload(&self) -> &Payload {
        &self.payload
    }
}

impl<Payload: Copy> Copy for Cmd<Payload> { }

impl<Payload: Clone> Clone for Cmd<Payload> {
    fn clone(&self) -> Self {
        Self::new(self.meta, self.payload.clone())
    }
}
