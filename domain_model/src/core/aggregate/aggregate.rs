use crate::core::*;

pub trait Aggregate : IdTypeDef + Default {
    type Kind;
    type SnapshotData;
    type CmdData;
    type EvtData;
    type Error;

    fn kind() -> Self::Kind;

    fn address(&self) -> AggregateAddress<Self> {
        AggregateAddress::<Self>::new(Self::kind(), self.id())
    }

    fn try_from(data: Self::SnapshotData) -> Result<Self, Self::Error>;
    fn into(&self) -> Self::SnapshotData;

    fn handle(&self, cmd: &Cmd<Self>) -> Result<Vec<Evt<Self>>, Self::Error>;
    fn apply(&mut self, evt: &Evt<Self>) -> Result<(), Self::Error>;
}
