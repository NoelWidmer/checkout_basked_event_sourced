use crate::core::*;

pub trait SnapshotStore<Agg: Aggregate> {
    fn retrieve_latest(&self, id: &Agg::Id) -> Result<Option<Snapshot<Agg>>, ()>;
}
