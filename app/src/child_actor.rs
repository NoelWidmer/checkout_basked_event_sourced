use actix::*;

pub struct ChildActor;

impl Actor for ChildActor {    
    type Context = Context<Self>;
}
