use crate::core::*;

pub trait SnapshotStore<Agg: Aggregate> {
    fn retrieve(&self, id: &Agg::Id) -> Result<Option<Snapshot<Agg>>, ()>;
    fn store(&self, snapshot: Snapshot<Agg>);
}
