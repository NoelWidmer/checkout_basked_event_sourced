use actix::*;

pub struct RootActor;

impl Actor for RootActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
       println!("I am alive!");
       System::current().stop(); // <- stop system
    }
}