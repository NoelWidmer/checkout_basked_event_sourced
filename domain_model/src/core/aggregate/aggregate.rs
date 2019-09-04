use crate::core::*;

pub trait Aggregate : IdTypeDef + Default {
    type SnapshotData;
    type CmdData;
    type EvtData;
    type Error;

    fn try_from(data: Self::SnapshotData) -> Result<Self, Self::Error>;
    fn into(&self) -> Self::SnapshotData;

    fn handle(&self, cmd: &Cmd<Self::CmdData>) -> Result<Vec<Evt<Self::EvtData>>, Self::Error>;
    fn apply(&mut self, evt: &Evt<Self::EvtData>) -> Result<(), Self::Error>;
}
