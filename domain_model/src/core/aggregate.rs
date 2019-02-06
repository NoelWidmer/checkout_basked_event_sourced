use super::Entity;
use super::AggregateRoot;

pub struct Aggregate<Inner: AggregateRoot> {
    root: Entity<Inner>
}

impl<Inner: AggregateRoot> Aggregate<Inner> {
    pub fn new(id: Inner::Id, inner: Inner) -> Self {
        Self {
            root: Entity::new(id, inner)
        }
    }

    pub fn id(&self) -> &Inner::Id {
        self.root.id()
    }
}

impl<Inner: AggregateRoot + Default> Aggregate<Inner> {
    pub fn new_default(id: Inner::Id) -> Self {
        Self {
            root: Entity::new_default(id)
        }
    }
}

impl<Inner: AggregateRoot> PartialEq for Aggregate<Inner> {
    fn eq(&self, other: &Aggregate<Inner>) -> bool {
        self.root.id() == other.root.id()
    }
}

impl<Inner: AggregateRoot> Eq for Aggregate<Inner> {
}



#[cfg(test)]
mod tests {
    use super::super::HasId;
    use super::Aggregate;
    use super::AggregateRoot;

    struct Test {
    }

    impl Test {
        fn new() -> Self {
            Self { }
        }
    }

    impl AggregateRoot for Test {

    }

    impl HasId for Test {
        type Id = usize;
    }
    
    #[test]
    fn aggregate_eq() {
        let a = Aggregate::new(99, Test::new());
        let b = Aggregate::new(99, Test::new());
        assert!(a == b);
    }
    
    #[test]
    fn aggregate_neq() {
        let a = Aggregate::new(77, Test::new());
        let b = Aggregate::new(88, Test::new());
        assert!(a != b);
    }
    
    #[test]
    fn aggregate_id() {
        let a = Aggregate::new(55, Test::new());
        assert!(*a.id() == 55);
    }
}
