use super::*;

pub trait Aggregate {
    type CmdData;
    type EvtData;
    type Error;

    fn handle(&self, cmd: &Cmd<Self::CmdData>) -> Result<Vec<Evt<Self::EvtData>>, Self::Error>;
    fn apply(&mut self, evt: &Evt<Self::EvtData>) -> Result<(), Self::Error>;
}
