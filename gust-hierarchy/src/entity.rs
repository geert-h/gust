#[derive(Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Entity(pub u32);

impl Entity {
    pub fn new(id: u32) -> Self {
        Entity(id)
    }
}