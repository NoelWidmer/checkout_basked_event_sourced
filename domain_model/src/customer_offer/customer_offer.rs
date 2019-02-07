use uuid::Uuid;
use crate::core::*;

pub struct CustomerOffer {

}

impl IdDefinition for CustomerOffer {
    type Id = Uuid;
}

impl AggregateRoot for CustomerOffer {
    type CommandData = super::commands::Command;
    type HandleError = super::HandleError;
    type EventData = super::events::Event;
    type ApplyError = super::ApplyError;

    fn handle(&self, command: Command<Self::CommandData>) -> Result<Vec<Event<Self::EventData>>, Self::HandleError> {
        match command.data() {
            super::commands::Command::AddItem(add_item) => self.remove_item(add_item), 
            super::commands::Command::RemoveItem(remove_item) => self.remove_item(remove_item)
        }
    }

    fn apply(&mut self, event: Event<Self::EventData>) -> Result<(), Self::ApplyError> {
        match event {

        }
    }
}

impl CustomerOffer {
    fn add_item(&mut self, add_item: AddItem) -> Result<Vec<Event<CustomerOffer::EventData>>, CustomerOffer::HandleError> {
        Ok(Vec::new())
    }

    fn remove_item(&mut self, remove_item: RemoveItem) -> Result<Vec<Event<Self::EventData>>, Self::HandleError> {
        Ok(Vec::new())
    }
}
