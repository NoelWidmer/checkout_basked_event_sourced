use crate::core::*;

pub enum AggregateError<Agg: Aggregate> {
    CouldNotRetrieveSnapshot,
    CouldNotHydrateFromSnapshot(Agg::Error),
    CouldNotRetrieveEvents,
    CouldNotStoreEvents, 
    CouldNotHandleCommand(Agg::Error), 
    CouldNotApplyEvent(Agg::Error),
    CorruptionDetected, 
}
