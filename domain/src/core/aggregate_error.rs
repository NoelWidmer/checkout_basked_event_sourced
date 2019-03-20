use super::*;

pub enum AggregateError<Root: Aggregate> {
    CouldNotRetrieveEvents,
    CouldNotStoreEvents, 
    CouldNotHandleCommand(Root::Error), 
    CouldNotApplyEvent(Root::Error),
    CorruptionDetected, 
}
