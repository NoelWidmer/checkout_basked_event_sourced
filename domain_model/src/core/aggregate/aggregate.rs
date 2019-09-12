use crate::core::*;

pub trait Aggregate : IdTypeDef + Default {
    type Kind;
    type SnapshotPayload;
    type CmdPayload;
    type EvtPayload;
    type Error;

    // ctor
    fn new_with_id(id: Self::Id) -> Self;
    fn try_from(data: Self::SnapshotPayload) -> Result<Self, Self::Error>;
    fn into(&self) -> Self::SnapshotPayload;

    // address
    fn kind() -> Self::Kind;

    fn address(&self) -> AggregateAddress<Self> {
        AggregateAddress::new(Self::kind(), self.id())
    }

    // messages
    fn handle(&self, cmd: &Cmd<Self>) -> Result<Vec<Evt<Self>>, Self::Error>;
    fn apply(&mut self, evt: &Evt<Self>) -> Result<(), Self::Error>;
}
