extern crate actix;

mod actor_system;
pub use actor_system::ActorSystem;

mod root_actor;
use root_actor::RootActor;

mod child_actor;
use child_actor::ChildActor;
