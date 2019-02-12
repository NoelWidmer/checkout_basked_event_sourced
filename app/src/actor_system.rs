use actix::*;

pub struct ActorSystem;

impl ActorSystem {
    pub fn start(name: &'static str) {
        let sys = actix::System::new(name);
        let actor = super::RootActor { addrs: Vec::new() };
        let addr = super::RootActor::start(actor);
        sys.run();
    }
}