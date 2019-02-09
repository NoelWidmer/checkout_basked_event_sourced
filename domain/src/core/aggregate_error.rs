use super::*;

pub enum AggregateError<Root: AggregateRoot> {
    CouldNotRetrieveEvents,
    CouldNotStoreEvents, 
    CouldNotHandleCommand(Root::Error), 
    CouldNotApplyEvent(Root::Error),
    CorruptionDetected, 
}
