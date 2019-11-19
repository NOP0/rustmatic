// TODO: Is slotmap the correct container for this, check out arrayvec. (Recent
// const generics evolution)
use slotmap::{new_key_type, Key, SlotMap};

/// Process image entities need to implement this trait.
pub trait ImageEntity: Clone + Copy {
    type KeyType: Key;
}

// Shorthands
pub type ImageEntityKey<E> = <E as ImageEntity>::KeyType;
pub type ImageEntitySlotMap<E> = SlotMap<<E as ImageEntity>::KeyType, E>;

// TODO: Current implementation is separate data structures for each data type,
// consider options.

// Impl for bool
new_key_type! {pub struct BoolKeyType;}

impl ImageEntity for bool {
    type KeyType = BoolKeyType;
}

pub type BoolKey = <bool as ImageEntity>::KeyType;

// Impl for f64
new_key_type! {pub struct FloatKeyType;}

impl ImageEntity for f64 {
    type KeyType = FloatKeyType;
}

pub type FloatKey = <f64 as ImageEntity>::KeyType;

// TODO: Process image outputs
pub struct ProcessImageInputs<E: ImageEntity> {
    image: ImageEntitySlotMap<E>,
}

impl<E> ProcessImageInputs<E>
where
    E: ImageEntity,
{
    pub fn new(size: usize) -> Self {
        ProcessImageInputs {
            image: SlotMap::with_capacity_and_key(size),
        }
    }

    pub fn insert(&mut self, entity: E) -> <E as ImageEntity>::KeyType {
        self.image.insert(entity)
    }

    pub fn get(&self, key: <E as ImageEntity>::KeyType) -> Option<&E> {
        self.image.get(key)
    }

    /// TODO: Poor ergonomics in user program
    pub fn get_mut(
        &mut self,
        key: <E as ImageEntity>::KeyType,
    ) -> Option<&mut E> {
        self.image.get_mut(key)
    }
}
