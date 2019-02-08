use super::*;
use std::sync::Arc;

pub struct Aggregate<Root: AggregateRoot + Default> {
    is_corrupt: bool,
    generation: u64,
    root: Entity<Root>, 
    store: Arc<EventStore<Root>>
}

impl<Root: AggregateRoot + Default> Aggregate<Root> {
    pub fn new(id: Root::Id, store: Arc<EventStore<Root>>) -> Result<Self, AggregateError<Root>> {
        let mut agg = Self {
            is_corrupt: false,
            generation: 0,
            root: Entity::new_default(id), 
            store
        };

        agg.hydrate().map(|()| agg)
    }

    pub fn id(&self) -> &Root::Id {
        self.root.id()
    }

    pub fn simulate(&self, cmd: &Cmd<Root::CmdData>) -> Result<Vec<Evt<Root::EvtData>>, AggregateError<Root>> {
        if self.is_corrupt {
            Err(AggregateError::CorruptionDetected)
        } else {
            self.root
                .inner()
                .handle(cmd)
                .map_err(|handle_err| AggregateError::CouldNotHandleCommand(handle_err))
        }
    }

    pub fn execute(&mut self, cmd: &Cmd<Root::CmdData>) -> Result<Vec<Evt<Root::EvtData>>, AggregateError<Root>> {
        self.simulate(cmd).and_then(|evts| {
            if let Err(()) = self.store.store(&evts, self.generation) {
                Err(AggregateError::CouldNotStoreEvents)
            } else {
                for evt in &evts {
                    self.apply_and_grow(evt)?;
                }

                Ok(evts)
            }
        })
    }

    fn apply_and_grow(&mut self, evt: &Evt<Root::EvtData>) -> Result<(), AggregateError<Root>> {
        match self.root.inner_mut().apply(evt) {
            Ok(()) => {
                // Event applied.
                self.generation += 1;
                Ok(())
            }, 
            Err(apply_err) => {
                // Apply failed. 
                self.is_corrupt = true;
                Err(AggregateError::CouldNotApplyEvent(apply_err))
            }
        }
    }

    fn hydrate(&mut self) -> Result<(), AggregateError<Root>> {
        match self.store.retrieve_all(self.root.id()) {
            Ok(evts) => {
                for evt in &evts {                    
                    self.apply_and_grow(evt)?;
                }

                Ok(())
            }, 
            Err(()) => Err(AggregateError::CouldNotRetrieveEvents)
        }
    }
}

impl<Root: AggregateRoot + Default> PartialEq for Aggregate<Root> {
    fn eq(&self, other: &Aggregate<Root>) -> bool {
        self.root.id() == other.root.id()
    }
}

impl<Root: AggregateRoot + Default> Eq for Aggregate<Root> {
}



/*#[cfg(test)]
mod tests {
    use super::super::HasId;
    use super::super::Command;
    use super::super::Event;
    use super::Aggregate;
    use super::AggregateRoot;

    struct Test {
    }

    impl Test {
        fn new() -> Self {
            Self { }
        }
    }

    impl AggregateRoot for Test {
        type CommandData = ();
        type HandleError = ();
        type EventData = ();
        type ApplyError = ();

        fn handle(&self, command: Command<()>) -> Result<Vec<Event<()>>, ()> {
            Err(())
        }

        fn apply(&self, event: Vec<Event<()>>) -> Result<(), ()> {
            Ok(())
        }
    }

    impl HasId for Test {
        type Id = usize;
    }
    
    #[test]
    fn aggregate_eq() {
        let a = Aggregate::new(99, Test::new());
        let b = Aggregate::new(99, Test::new());
        assert!(a == b);
    }
    
    #[test]
    fn aggregate_neq() {
        let a = Aggregate::new(77, Test::new());
        let b = Aggregate::new(88, Test::new());
        assert!(a != b);
    }
    
    #[test]
    fn aggregate_id() {
        let a = Aggregate::new(55, Test::new());
        assert!(*a.id() == 55);
    }
}*/
