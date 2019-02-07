use super::*;

pub trait AggregateRoot : IdDefinition {
    type CommandData;
    type HandleError;
    type EventData;
    type ApplyError;

    fn handle(&self, command: Command<Self::CommandData>) -> Result<Vec<Event<Self::EventData>>, Self::HandleError>;
    fn apply(&mut self, event: Event<Self::EventData>) -> Result<(), Self::ApplyError>;
}
