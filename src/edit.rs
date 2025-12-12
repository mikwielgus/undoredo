use crate::collection::{Insert, IntoIter, Keyed, Remove};

pub struct Edit<EC> {
    pub(super) removed: EC,
    pub(super) inserted: EC,
}

impl<EC: Default> Edit<EC> {
    pub fn new() -> Self {
        Self {
            removed: Default::default(),
            inserted: Default::default(),
        }
    }
}

impl<EC> Edit<EC> {
    pub fn reverse(self) -> Self {
        Self {
            removed: self.inserted,
            inserted: self.removed,
        }
    }
}

impl<EC: Default> Default for Edit<EC> {
    fn default() -> Self {
        Self {
            removed: Default::default(),
            inserted: Default::default(),
        }
    }
}

pub trait ApplyEdit<EC> {
    fn apply_edit(&mut self, edit: &Edit<EC>);
}

impl<
    K: Clone,
    V: Clone,
    C: Insert<K, Item = V> + Remove<K, Item = V>,
    EC: Clone + IntoIter<K, Item = V> + Keyed<Key = K>,
> ApplyEdit<EC> for C
{
    fn apply_edit(&mut self, edit: &Edit<EC>) {
        for (removed_key, _removed_value) in edit.removed.clone().into_iter() {
            self.remove(&removed_key);
        }

        for (inserted_key, inserted_value) in edit.inserted.clone().into_iter() {
            self.insert(inserted_key.clone(), inserted_value.clone());
        }
    }
}
