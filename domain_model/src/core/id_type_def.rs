pub trait IdTypeDef {
    type Id: Eq;

    fn id(&self) -> Self::Id;
}