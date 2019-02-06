use super::HasId;

pub struct Entity<Inner: HasId> {
    id: Inner::Id,
    inner: Inner
}

impl<Inner: HasId> Entity<Inner> {
    fn new(id: Inner::Id, inner: Inner) -> Self {
        Self {
            id, 
            inner
        }
    }

    fn id(&self) -> &Inner::Id {
        &self.id
    }

    fn inner(&self) -> &Inner {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Inner {
        &mut self.inner
    }
}

impl<Inner: HasId> PartialEq for Entity<Inner> {
    fn eq(&self, other: &Entity<Inner>) -> bool {
        self.id == other.id
    }
}

impl<Inner: HasId> Eq for Entity<Inner> {
}

#[cfg(test)]
mod tests {
    use super::Entity;
    use super::HasId;

    struct Test {
    }

    impl Test {
        fn new() -> Self {
            Self { }
        }
    }

    impl HasId for Test {
        type Id = usize;
    }
    
    #[test]
    fn entity_eq() {
        let a = Entity::new(99, Test::new());
        let b = Entity::new(99, Test::new());
        assert!(a == b);
    }
    
    #[test]
    fn entity_neq() {
        let a = Entity::new(77, Test::new());
        let b = Entity::new(88, Test::new());
        assert!(a != b);
    }
}
