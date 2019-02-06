use super::*;

pub trait AggregateRoot : HasId {
    type CommandData;
    type HandleError;
    type EventData;
    type ApplyError;

    fn handle(&self, command: Command<Self::CommandData>) -> Result<Vec<Event<Self::EventData>>, Self::HandleError>;
    fn apply(&self, event: Event<Self::EventData>) -> Result<(), Self::ApplyError>;
}