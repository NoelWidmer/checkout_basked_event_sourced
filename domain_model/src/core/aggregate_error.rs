use super::*;

pub enum AggregateError<Root: AggregateRoot> {
    CouldNotRetrieveEvents,
    CouldNotStoreEvents, 
    CouldNotHandleCommand(Root::HandleError), 
    CouldNotApplyEvent(Root::ApplyError),
    CorruptionDetected, 
}
