use super::*;

pub trait EventStore<AggregateKind: AggregateRoot> {
    fn store(&mut self, events: &Vec<Event<AggregateKind::EventData>>, expected_generation: u64) -> Result<(), ()>;
}