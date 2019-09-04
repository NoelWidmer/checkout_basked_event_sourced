use crate::core::*;

pub struct Snapshot<Agg: Aggregate> {
    generation: u64,
    payload: Agg::SnapshotData,
}

impl<Agg: Aggregate> Snapshot<Agg> {
    pub fn new(generation: u64, payload: Agg::SnapshotData) -> Self {
        Self {
            generation,
            payload,
        }
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn payload(self) -> Agg::SnapshotData {
        self.payload
    }
}

/*impl<Agg: Aggregate + Copy> Copy for Snapshot<Agg> { }

impl<Agg: Aggregate + Clone> Clone for Snapshot<Agg> {
    fn clone(&self) -> Self {
        Self::new(self.id, self.generation, self.payload.clone())
    }
}*/
