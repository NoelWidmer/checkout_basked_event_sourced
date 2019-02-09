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

    pub fn meta(&self) -> MsgMeta {
        self.meta
    }

    pub fn data(&self) -> &Data {
        &self.data
    }
}

impl<Data: Copy> Copy for Evt<Data> { }

impl<Data: Clone> Clone for Evt<Data> {
    fn clone(&self) -> Self {
        Self::new(self.meta, self.data.clone())
    }
}
