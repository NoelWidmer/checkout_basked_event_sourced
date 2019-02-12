pub struct ActorSystem;

impl ActorSystem {
    pub fn start(name: &'static str) {
        let sys = actix::System::new(name);        
        let addr = super::RootActor.start();
        sys.run();
    }
}