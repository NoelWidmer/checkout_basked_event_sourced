use super::MsgMeta;

pub struct Evt<Data> {
    meta: MsgMeta,
    data: Data,
}

impl<Data> Evt<Data> {
    pub fn new(meta: MsgMeta, data: Data) -> Self {
        Self {
            meta,
            data,
        }
    }

    pub fn meta(&self) -> &MsgMeta {
        &self.meta
    }

    pub fn data(&self) -> &Data {
        &self.data
    }
}
