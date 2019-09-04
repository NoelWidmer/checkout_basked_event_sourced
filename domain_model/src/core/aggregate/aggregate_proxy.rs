use crate::core::*;
use std::sync::Arc;

pub struct AggregateProxy<Agg: Aggregate> {
    is_corrupt: bool,
    generation: u64,
    entity_proxy: EntityProxy<Agg>, 
    snapshot_store: Arc<SnapshotStore<Agg>>, 
    event_store: Arc<EvtStore<Agg>>,
}

impl<Agg: Aggregate> AggregateProxy<Agg> {
    pub fn new(
        id: Agg::Id, 
        snapshot_store: Arc<SnapshotStore<Agg>>, 
        event_store: Arc<EvtStore<Agg>>) 
        -> Result<Self, AggregateError<Agg>> {

        let (generation, entity_proxy) = Self::hydrate_from_snapshot(id, &snapshot_store)?;

        let mut aggregate_proxy = AggregateProxy {
            is_corrupt: false,
            generation,
            entity_proxy, 
            snapshot_store, 
            event_store
        };

        aggregate_proxy.hydrate_from_events()?;
        Ok(aggregate_proxy)
    }

    fn hydrate_from_snapshot(id: Agg::Id, snapshot_store: &Arc<SnapshotStore<Agg>>) 
    -> Result<(u64, EntityProxy<Agg>), AggregateError<Agg>> {
        match snapshot_store.retrieve_latest(&id) {
            Ok(Some(snapshot)) => {
                let gen = snapshot.generation();

                match Agg::try_from(snapshot.data()) {
                    Ok(aggregate) => Ok((gen, EntityProxy::new(id, aggregate))), 
                    Err(err) => Err(AggregateError::CouldNotHydrateFromSnapshot(err))
                }
            }, 
            Ok(None) => Ok((0, EntityProxy::new_default(id))), 
            Err(()) => Err(AggregateError::CouldNotRetrieveSnapshot)
        }
    }
    
    fn hydrate_from_events(&mut self) -> Result<(), AggregateError<Agg>> {        
        match self.event_store.retrieve_all(self.entity_proxy.id()) {
            Ok(evts) => {
                for evt in &evts {                    
                    self.apply_and_grow(evt)?;
                }

                Ok(())
            }, 
            Err(()) => Err(AggregateError::CouldNotRetrieveEvents)
        }
    }

    fn apply_and_grow(&mut self, evt: &Evt<Agg::EvtData>) -> Result<(), AggregateError<Agg>> {
        match self.entity_proxy.entity_mut().apply(evt) {
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

    pub fn id(&self) -> &Agg::Id {
        self.entity_proxy.id()
    }

    pub fn simulate(&self, cmd: &Cmd<Agg::CmdData>) -> Result<Vec<Evt<Agg::EvtData>>, AggregateError<Agg>> {
        if self.is_corrupt {
            Err(AggregateError::CorruptionDetected)
        } else {
            self.entity_proxy
                .entity()
                .handle(cmd)
                .map_err(|handle_err| AggregateError::CouldNotHandleCommand(handle_err))
        }
    }

    pub fn execute(&mut self, cmd: &Cmd<Agg::CmdData>) -> Result<Vec<Evt<Agg::EvtData>>, AggregateError<Agg>> {
        self.simulate(cmd).and_then(|evts| {
            if evts.len() > 0 {
                if let Err(()) = self.event_store.store(&evts, self.generation) {
                    Err(AggregateError::CouldNotStoreEvents)
                } else {
                    for evt in &evts {
                        self.apply_and_grow(evt)?;
                    }

                    Ok(evts)
                }
            } else {
                Ok(evts)
            }
        })
    }
}

impl<Agg: Aggregate> PartialEq for AggregateProxy<Agg> {
    fn eq(&self, other: &AggregateProxy<Agg>) -> bool {
        self.entity_proxy.id() == other.entity_proxy.id()
    }
}

impl<Agg: Aggregate> Eq for AggregateProxy<Agg> {
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
