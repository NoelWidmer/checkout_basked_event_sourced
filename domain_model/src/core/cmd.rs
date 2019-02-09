use super::MsgMeta;

pub struct Cmd<Data> {
    meta: MsgMeta, 
    data: Data,
}

impl<Data> Cmd<Data> {
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

impl<Data: Copy> Copy for Cmd<Data> { }

impl<Data: Clone> Clone for Cmd<Data> {
    fn clone(&self) -> Self {
        Self::new(self.meta, self.data.clone())
    }
}
