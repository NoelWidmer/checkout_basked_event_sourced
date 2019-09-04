use crate::core::*;

pub trait EvtStore<Agg: Aggregate> {
    fn retrieve_all(&self, id: &Agg::Id) -> Result<Vec<Evt<Agg::EvtData>>, ()>;
    fn store(&self, evts: &Vec<Evt<Agg::EvtData>>, expected_generation: u64) -> Result<(), ()>;
}
