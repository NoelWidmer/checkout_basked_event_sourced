use actix::*;

pub struct RootActor {
    pub addrs: Vec<Addr<super::ChildActor>>
}

impl Actor for RootActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("root actor started.");

        for i in 0..1_000_000 {
            let addr = super::ChildActor.start();
            self.addrs.push(addr);
        }
        
        println!("done.");
    }
}