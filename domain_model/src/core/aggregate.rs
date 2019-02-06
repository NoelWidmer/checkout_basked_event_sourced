use super::*;

pub struct Aggregate<Inner: AggregateRoot> {
    is_corrupt: bool,
    generation: u64,
    root: Entity<Inner>, 
    store: Box<EventStore<Inner>>
}

impl<Inner: AggregateRoot> Aggregate<Inner> {
    pub fn new_from_template(id: Inner::Id, template: Inner, store: Box<EventStore<Inner>>) -> Self {
        Self {
            is_corrupt: false,
            generation: 0,
            root: Entity::new(id, template), 
            store
        }
    }

    pub fn id(&self) -> &Inner::Id {
        self.root.id()
    }

    pub fn simulate(&self, command: Command<Inner::CommandData>) -> Result<Vec<Event<Inner::EventData>>, SimulateError<Inner::HandleError>> {
        if self.is_corrupt {
            Err(SimulateError::CorruptionDetected)
        } else {
            match self.root.inner().handle(command) {
                Ok(events) => Ok(events), 
                Err(err) => Err(SimulateError::CouldNotHandle(err))
            }
        }
    }

    pub fn execute(&mut self, command: Command<Inner::CommandData>) -> Result<(), ExecuteError<Inner::HandleError, Inner::ApplyError>> {
        match self.simulate(command) {
            Ok(events) => {
                match self.store.store(&events, self.generation) {
                    Ok(()) => {
                        // Events stored.
                    }, 
                    Err(()) => return Err(ExecuteError::CouldNotStore)
                }

                for event in events {                
                    match self.root.inner_mut().apply(event) {
                        Ok(()) => {
                            // Event applied.
                            self.generation += 1;
                        }, 
                        Err(err) => {
                            // Apply failed. We have just peristed corrupt data. This is bad!
                            self.is_corrupt = true;
                            return Err(ExecuteError::CouldNotApply(err))
                        }
                    }
                }

                Ok(())
            }, 
            Err(simulate_err) => {
                // Could not handle.
                match simulate_err {
                    SimulateError::CorruptionDetected => Err(ExecuteError::CorruptionDetected),
                    SimulateError::CouldNotHandle(handle_err) => Err(ExecuteError::CouldNotHandle(handle_err))
                }
            }
        }
    }

    fn hydrate(&mut self) {
        // TODO hydrate
    }
}

impl<Inner: AggregateRoot + Default> Aggregate<Inner> {
    pub fn new(id: Inner::Id, store: Box<EventStore<Inner>>) -> Self {
        let mut agg = Self {
            is_corrupt: false,
            generation: 0,
            root: Entity::new_default(id), 
            store
        };

        agg.hydrate();
        return agg;
    }
}

impl<Inner: AggregateRoot> PartialEq for Aggregate<Inner> {
    fn eq(&self, other: &Aggregate<Inner>) -> bool {
        self.root.id() == other.root.id()
    }
}

impl<Inner: AggregateRoot> Eq for Aggregate<Inner> {
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
