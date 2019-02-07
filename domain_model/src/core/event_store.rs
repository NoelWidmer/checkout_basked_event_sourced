use super::*;

pub trait EventStore<AggregateKind: AggregateRoot> {
    fn retrieve_all(&self, id: &AggregateKind::Id) -> Result<Vec<Event<AggregateKind::EventData>>, ()>;
    fn store(&self, events: &Vec<Event<AggregateKind::EventData>>, expected_generation: u64) -> Result<(), ()>;
}