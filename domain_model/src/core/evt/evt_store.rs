use crate::core::*;

pub trait EvtStore<Agg: Aggregate> {
    fn retrieve(&self, id: &Agg::Id, first_event_index: u64) -> Result<Vec<Evt<Agg>>, ()>;
    fn store(&self, evts: &Vec<Evt<Agg>>, expected_generation: u64) -> Result<(), ()>;
    
    fn retrieve_all(&self, id: &Agg::Id) -> Result<Vec<Evt<Agg>>, ()> {
        self.retrieve(id, 0)
    }
}
