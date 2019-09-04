use crate::core::*;

pub struct EntityProxy<Entity: IdTypeDef> {
    entity: Entity
}

impl<Entity: IdTypeDef> EntityProxy<Entity> {
    pub fn new(id: Entity::Id, entity: Entity) -> Self {
        Self {
            id, 
            entity
        }
    }

    pub fn id(&self) -> &Entity::Id {
        &self.id
    }

    pub fn entity(&self) -> &Entity {
        &self.entity
    }

    pub fn entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }
}

impl<Entity: IdTypeDef + Default> EntityProxy<Entity> {
    pub fn new_default(id: Entity::Id) -> Self {
        EntityProxy {
            id, 
            entity: Entity::default()
        }
    }
}

impl<Entity: IdTypeDef> PartialEq for EntityProxy<Entity> {
    fn eq(&self, other: &EntityProxy<Entity>) -> bool {
        self.id == other.id
    }
}

impl<Entity: IdTypeDef> Eq for EntityProxy<Entity> {
}



#[cfg(test)]
mod tests {
    use super::EntityProxy;
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
        let a = EntityProxy::new(99, UsizeTest::new());
        let b = EntityProxy::new(99, UsizeTest::new());
        assert!(a == b);
    }
    
    #[test]
    fn entity_copy_neq() {
        let a = EntityProxy::new(77, UsizeTest::new());
        let b = EntityProxy::new(88, UsizeTest::new());
        assert!(a != b);
    }
    
    #[test]
    fn entity_id() {
        let a = EntityProxy::new(55, UsizeTest::new());
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
        let a = EntityProxy::new("hello".to_string(), StringTest::new());
        let b = EntityProxy::new("hello".to_string(), StringTest::new());
        assert!(a == b);
    }
    
    #[test]
    fn entity_non_copy_neq() {
        let a = EntityProxy::new("hello".to_string(), StringTest::new());
        let b = EntityProxy::new("world".to_string(), StringTest::new());
        assert!(a != b);
    }
}
