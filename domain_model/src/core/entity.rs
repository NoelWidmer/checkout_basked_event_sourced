pub trait Entity<Id: Eq> {
    fn id(&self) -> &Id;
}

impl<T: Entity<Id>, Id: Eq> PartialEq for T {
    fn eq(&self, other: &T) -> bool {
        self.id() == other.id()
    }
}

impl<T: Entity<Id>, Id: Eq> Eq for T {
}

#[cfg(test)]
mod tests {
    struct TestEntity {
        id: String
    }

    impl TestEntity {
        pub fn new(id: String) -> Self {
            Self {
                id
            }
        }
    }

    impl<'a> super::Entity<String> for TestEntity {
        fn id(&self) -> &String {
            &self.id
        }
    }
    
    #[test]
    fn entity_equals() {
        let a = TestEntity::new("hello".to_string());
        let b = TestEntity::new("hello".to_string());
        assert_eq!(a, b);
    }
}
