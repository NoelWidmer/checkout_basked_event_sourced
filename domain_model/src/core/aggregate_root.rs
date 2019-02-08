use super::*;

pub trait AggregateRoot : IdTypeDef {
    type CmdData;
    type HandleError;
    type EvtData;
    type ApplyError;

    fn handle(&self, cmd: &Cmd<Self::CmdData>) -> Result<Vec<Evt<Self::EvtData>>, Self::HandleError>;
    fn apply(&mut self, evt: &Evt<Self::EvtData>) -> Result<(), Self::ApplyError>;
}
