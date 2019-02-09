use super::*;

pub struct Entity<Inner: IdTypeDef> {
    id: Inner::Id,
    inner: Inner
}

impl<Inner: IdTypeDef> Entity<Inner> {
    pub fn new(id: Inner::Id, inner: Inner) -> Self {
        Self {
            id, 
            inner
        }
    }

    pub fn id(&self) -> &Inner::Id {
        &self.id
    }

    pub fn inner(&self) -> &Inner {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Inner {
        &mut self.inner
    }
}

impl<Inner: IdTypeDef + Default> Entity<Inner> {
    pub fn new_default(id: Inner::Id) -> Self {
        Self {
            id, 
            inner: Inner::default()
        }
    }
}

impl<Inner: IdTypeDef> PartialEq for Entity<Inner> {
    fn eq(&self, other: &Entity<Inner>) -> bool {
        self.id == other.id
    }
}

impl<Inner: IdTypeDef> Eq for Entity<Inner> {
}



#[cfg(test)]
mod tests {
    use super::Entity;
    use super::IdTypeDef;

    struct UsizeTest {
    }

    impl UsizeTest {
        fn new() -> Self {
            Self { }
        }
    }

    impl IdTypeDef for UsizeTest {
        type Id = usize;
    }
    
    #[test]
    fn entity_copy_eq() {
        let a = Entity::new(99, UsizeTest::new());
        let b = Entity::new(99, UsizeTest::new());
        assert!(a == b);
    }
    
    #[test]
    fn entity_copy_neq() {
        let a = Entity::new(77, UsizeTest::new());
        let b = Entity::new(88, UsizeTest::new());
        assert!(a != b);
    }
    
    #[test]
    fn entity_id() {
        let a = Entity::new(55, UsizeTest::new());
        assert!(*a.id() == 55);
    }
    
    struct StringTest {
    }

    impl StringTest {
        fn new() -> Self {
            Self { }
        }
    }

    impl IdTypeDef for StringTest {
        type Id = String;
    }
    
    #[test]
    fn entity_non_copy_eq() {
        let a = Entity::new("hello".to_string(), StringTest::new());
        let b = Entity::new("hello".to_string(), StringTest::new());
        assert!(a == b);
    }
    
    #[test]
    fn entity_non_copy_neq() {
        let a = Entity::new("hello".to_string(), StringTest::new());
        let b = Entity::new("world".to_string(), StringTest::new());
        assert!(a != b);
    }
}
