use crate::core::*;

pub struct Snapshot<Agg: Aggregate> {
    generation: u64,
    data: Agg::SnapshotData,
}

impl<Agg: Aggregate> Snapshot<Agg> {
    pub fn new(generation: u64, data: Agg::SnapshotData) -> Self {
        Self {
            generation,
            data,
        }
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn data(self) -> Agg::SnapshotData {
        self.data
    }
}

/*impl<Agg: Aggregate + Copy> Copy for Snapshot<Agg> { }

impl<Agg: Aggregate + Clone> Clone for Snapshot<Agg> {
    fn clone(&self) -> Self {
        Self::new(self.id, self.generation, self.data.clone())
    }
}*/
