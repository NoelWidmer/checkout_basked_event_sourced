use super::*;

pub trait EventStore<AggregateKind: Aggregate + IdTypeDef> {
    fn retrieve_all(&self, id: &AggregateKind::Id) -> Result<Vec<Evt<AggregateKind::EvtData>>, ()>;
    fn store(&self, evts: &Vec<Evt<AggregateKind::EvtData>>, expected_generation: u64) -> Result<(), ()>;
}
