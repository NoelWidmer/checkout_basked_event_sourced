use crate::core::*;

#[derive(Clone, Copy)]
pub struct AggregateAddress<Agg: Aggregate> {
    kind: Agg::Kind, 
    id: Agg::Id
}

impl<Agg: Aggregate> AggregateAddress<Agg> {
    pub fn new(kind: Agg::Kind, id: Agg::Id) -> AggregateAddress<Agg> {
        Self {
            kind, 
            id
        }
    }
}

/*
impl<Agg: Aggregate> Copy for AggregateAddress<Agg> where Agg::Kind: Copy, Agg::Id: Copy { }

impl<Agg: Aggregate> Clone for AggregateAddress<Agg> where Agg::Kind: Clone, Agg::Id: Clone { }*/
