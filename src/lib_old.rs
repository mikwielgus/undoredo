pub struct Recorder<K, V, C, EC = std::collections::HashMap<K, V>> {
    container: C,
    edit: Edit<EC>,
    key_marker: std::marker::PhantomData<K>,
    value_marker: std::marker::PhantomData<V>,
}

impl<K, V, C, EC: Default> Recorder<K, V, C, EC> {
    pub fn new(container: C) -> Self {
        Self {
            container,
            edit: Default::default(),
            key_marker: std::marker::PhantomData,
            value_marker: std::marker::PhantomData,
        }
    }

    pub fn dissolve(self) -> (C, Edit<EC>) {
        (self.container, self.edit)
    }
}

impl<K, V, C: Default, EC: Default> Default for Recorder<K, V, C, EC> {
    fn default() -> Self {
        Self {
            container: Default::default(),
            edit: Default::default(),
            key_marker: std::marker::PhantomData,
            value_marker: std::marker::PhantomData,
        }
    }
}

impl<K, V, C: cc_traits::Collection, EC> cc_traits::Collection for Recorder<K, V, C, EC> {
    type Item = C::Item;
}

impl<K, V, C: cc_traits::SimpleCollectionRef<Item = V>, EC> cc_traits::CollectionRef
    for Recorder<K, V, C, EC>
{
    type ItemRef<'a>
        = &'a V
    where
        Self: 'a;

    cc_traits::covariant_item_ref!();
}

impl<K, V, C: cc_traits::SimpleCollectionRef<Item = V>, EC> cc_traits::SimpleCollectionRef
    for Recorder<K, V, C, EC>
{
    cc_traits::simple_collection_ref!();
}

impl<K, V, C: cc_traits::Keyed, EC> cc_traits::Keyed for Recorder<K, V, C, EC> {
    type Key = K;
}

impl<K, V, C: cc_traits::SimpleKeyedRef, EC> cc_traits::KeyedRef for Recorder<K, V, C, EC> {
    type KeyRef<'a>
        = &'a K
    where
        Self: 'a;

    cc_traits::covariant_key_ref!();
}

impl<K, V, C: cc_traits::SimpleKeyedRef, EC> cc_traits::SimpleKeyedRef for Recorder<K, V, C, EC> {
    cc_traits::simple_keyed_ref!();
}

impl<K, V, C: cc_traits::Len, EC> cc_traits::Len for Recorder<K, V, C, EC> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.container.len()
    }
}

impl<Q, K, V, C: cc_traits::Get<Q> + cc_traits::SimpleCollectionRef<Item = V>, EC> cc_traits::Get<Q>
    for Recorder<K, V, C, EC>
{
    #[inline(always)]
    fn get(&self, key: Q) -> Option<&V> {
        Some(C::into_ref(self.container.get(key)?))
    }
}

impl<
    Q,
    K,
    V,
    C: cc_traits::GetKeyValue<Q>
        + cc_traits::SimpleCollectionRef<Item = V>
        + cc_traits::SimpleKeyedRef<Key = K>,
    EC,
> cc_traits::GetKeyValue<Q> for Recorder<K, V, C, EC>
{
    #[inline(always)]
    fn get_key_value(&self, key: Q) -> Option<(&K, &V)> {
        let (key, value) = self.container.get_key_value(key)?;

        Some((
            <C as cc_traits::SimpleKeyedRef>::into_ref(key),
            <C as cc_traits::SimpleCollectionRef>::into_ref(value),
        ))
    }
}

impl<K, V, C: cc_traits::Capacity, EC> cc_traits::Capacity for Recorder<K, V, C, EC> {
    #[inline(always)]
    fn capacity(&self) -> usize {
        self.container.capacity()
    }
}

impl<K, V, C: cc_traits::Reserve, EC> cc_traits::Reserve for Recorder<K, V, C, EC> {
    #[inline(always)]
    fn reserve(&mut self, additional: usize) {
        self.container.reserve(additional)
    }
}

impl<
    K: Clone,
    V: Clone,
    C: cc_traits::PushBack<Item = V, Output = K>,
    EC: cc_traits::MapInsert<K, Item = V>,
> cc_traits::PushBack for Recorder<K, V, C, EC>
{
    type Output = K;

    #[inline(always)]
    fn push_back(&mut self, value: V) -> K {
        let key = self.container.push_back(value.clone());
        self.edit.inserted.insert(key.clone(), value);

        key
    }
}

impl<
    V: Clone,
    C: cc_traits::Len + cc_traits::PopBack<Item = V>,
    EC: cc_traits::MapInsert<usize, Item = V>,
> cc_traits::PopBack for Recorder<usize, V, C, EC>
{
    #[inline(always)]
    fn pop_back(&mut self) -> Option<V> {
        let key = self.container.len();
        let value = self.container.pop_back()?;
        self.edit.removed.insert(key, value.clone());

        Some(value)
    }
}

impl<
    K: Clone,
    V: Clone,
    C: cc_traits::Get<K>
        + cc_traits::MapInsert<K, Item = V, Output = Option<V>>
        + cc_traits::SimpleCollectionRef<Item = V>,
    EC: cc_traits::MapInsert<K, Item = V>,
> cc_traits::MapInsert<K> for Recorder<K, V, C, EC>
{
    type Output = Option<V>;

    #[inline(always)]
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(removed_value) = self.container.get(key.clone()) {
            self.edit
                .removed
                .insert(key.clone(), C::into_ref(removed_value).clone());
        }

        self.edit.inserted.insert(key.clone(), value.clone());
        self.container.insert(key, value)
    }
}

/*impl<
    'a,
    K: Clone + 'a,
    V: Clone,
    C: cc_traits::Remove<&'a K, Item = V>,
    EC: cc_traits::MapInsert<K, Item = V> + cc_traits::Remove<K, Item = V>,
> cc_traits::Remove<K> for Recorder<K, V, C, EC>
{
    #[inline(always)]
    fn remove(&mut self, key: K) -> Option<V> {
        let value = self.container.remove(&key)?;
        self.edit.removed.insert(key.clone(), value.clone());

        Some(value)
    }
}*/

impl<
    'a,
    K: ToOwned,
    V: Clone,
    C: cc_traits::Remove<&'a K, Item = V>,
    EC: cc_traits::MapInsert<K::Owned, Item = V> + cc_traits::Remove<K::Owned, Item = V>,
> cc_traits::Remove<&'a K> for Recorder<K, V, C, EC>
{
    #[inline(always)]
    fn remove(&mut self, key: &'a K) -> Option<V> {
        let value = self.container.remove(key)?;
        self.edit.removed.insert(key.to_owned(), value.clone());

        Some(value)
    }
}

impl<K, V, C: cc_traits::Clear, EC> cc_traits::Clear for Recorder<K, V, C, EC> {
    #[inline(always)]
    fn clear(&mut self) {
        self.container.clear()
    }
}

#[cfg(test)]
mod tests {
    use cc_traits::{Clear, Get, MapInsert, Remove};
    use std::collections::BTreeMap;

    use crate::*;

    #[test]
    fn test_apply_edit() {
        let mut recorder = Recorder::new(BTreeMap::new());
        recorder.insert(1, 10);
        recorder.insert(2, 20);
        recorder.insert(3, 30);
        recorder.insert(4, 40);
        recorder.insert(5, 50);

        /*let edit = Edit {
            removed: vec![(2, 10)],
            inserted: vec![(3, 33), (6, 60)],
        };
        recorder.apply_edit(edit);

        assert_eq!(recorder.get(&1), Some(&10));
        assert!(!recorder.contains(&2));
        assert_eq!(recorder.get(&3), Some(&33));
        assert_eq!(recorder.get(&4), Some(&40));
        assert_eq!(recorder.get(&5), Some(&50));
        assert_eq!(recorder.get(&6), Some(&60));*/
    }

    #[test]
    fn test_recorder_insert_and_remove() {
        let mut recorder = Recorder::new(BTreeMap::new());

        recorder.insert(1, 10);
        recorder.insert(2, 20);
        recorder.insert(3, 30);
        recorder.insert(4, 40);
        recorder.insert(5, 50);
        recorder.remove(&2);

        assert_eq!(recorder.get(&1), Some(&10));
        assert!(!recorder.contains(&2));
        assert_eq!(recorder.get(&3), Some(&30));
        assert_eq!(recorder.get(&4), Some(&40));
        assert_eq!(recorder.get(&5), Some(&50));
    }

    #[test]
    fn test_clear() {
        let mut recorder = Recorder::new(BTreeMap::new());

        recorder.insert(1, 10);
        recorder.insert(2, 20);
        recorder.insert(3, 30);
        recorder.insert(4, 40);
        recorder.insert(5, 50);

        recorder.clear();

        assert!(recorder.container.is_empty());
    }
}

/*impl<K, V, C: cc_traits::Iter<Item = V> + cc_traits::SimpleCollectionRef<Item = V>, EC>
    cc_traits::Iter for Recorder<K, V, C, EC>
{
    type Iter<'a>
        = C::Iter<'a>
    where
        Self: 'a;

    #[inline(always)]
    fn iter(&self) -> Self::Iter<'_> {}
}*/

//impl<

/*pub struct Recorder<K, V, C, EC> {
    container: C,
    edit: Edit<EC>,
    key_marker: std::marker::PhantomData<K>,
    value_marker: std::marker::PhantomData<V>,
}

impl<K, V, C: cc_traits::Collection, EC> cc_traits::Collection for Recorder<K, V, C, EC> {
    type Item = V;
}

impl<K, V, C: cc_traits::CollectionRef, EC> cc_traits::CollectionRef for Recorder<K, V, C, EC> {
    type ItemRef<'a>
        = &'a V
    where
        Self: 'a;

    cc_traits::covariant_item_ref!();
}

impl<
    'a,
    Q,
    K,
    V: 'a,
    C: cc_traits::SimpleCollectionRef + cc_traits::Get<&'a Q, ItemRef<'a> = &'a V> + 'a,
    EC,
> cc_traits::Get<&'a Q> for Recorder<K, V, C, EC>
{
    fn get(&self, key: &'a Q) -> Option<&V> {
        Some(C::into_ref(self.container.get(key)?))
    }
}*/

/*pub struct Recorder<C, EC> {
    container: C,
    edit: Edit<EC>,
}

impl<C: cc_traits::Collection, EC> cc_traits::Collection for Recorder<C, EC> {
    type Item = C::Item;
}

impl<C: cc_traits::CollectionRef, EC> cc_traits::CollectionRef for Recorder<C, EC> {
    /*type ItemRef<'a>
        = &'a C::Item
    where
        Self: 'a;*/
    type ItemRef<'a>
        = C::ItemRef<'a>
    where
        Self: 'a;

    fn upcast_item_ref<'short, 'long: 'short>(r: Self::ItemRef<'long>) -> Self::ItemRef<'short>
    where
        Self: 'long,
    {
        r
    }
    //fn upcast_item_ref<'s
    //cc_traits::covariant_item_ref!();
}

impl<K, C: cc_traits::Get<K>, EC> cc_traits::Get<K> for Recorder<C, EC> {
    fn get(&self, key: K) -> Option<Self::ItemRef<'_>> {
        self.container.get(key)
    }
}*/

/*pub struct Recorder<K, V, C, EC> {
    container: C,
    edit: Edit<EC>,
    key_marker: std::marker::PhantomData<K>,
    value_marker: std::marker::PhantomData<V>,
}

impl<K, V, C, EC> cc_traits::Collection for Recorder<K, V, C, EC> {
    type Item = V;
}

impl<K, V, C, EC> cc_traits::CollectionRef for Recorder<K, V, C, EC> {
    type ItemRef<'a>
        = &'a V
    where
        Self: 'a;

    cc_traits::covariant_item_ref!();
}

impl<'a, Q, K, V, C: cc_traits::Get<&'a Q, ItemRef<'a> = &'a V>, EC> cc_traits::Get<&'a Q>
    for Recorder<K, V, C, EC>
{
    fn get(&self, key: &'a Q) -> Option<&V> {
        self.container.get(key)
    }
}*/

/*impl<K: Clone, V: Clone, C: Get<K, V> + Insert<K, V>, EC: Insert<K, V>> Insert<K, V>
    for Recorder<C, EC>
{
    fn insert(&mut self, key: K, value: V) {
        if let Some(old_value) = self.container.get(&key) {
            self.edit.olds.insert(key.clone(), old_value.clone());
        }

        self.container.insert(key.clone(), value.clone());
        self.edit.news.insert(key, value);
    }
}

impl<K: Clone, V: Clone, C: Remove<K, V>, EC: Insert<K, V>> Remove<K, V> for Recorder<C, EC> {
    fn remove(&mut self, key: K) -> Option<V> {
        let value = self.container.remove(key.clone())?;
        self.edit.olds.insert(key, value.clone());

        Some(value)
    }
}

impl<K: Clone, V: Clone, C: PushFront<K, V>, EC: Insert<K, V>> PushFront<K, V> for Recorder<C, EC> {
    fn push_front(&mut self, value: V) -> K {
        let key = self.container.push_front(value.clone());
        self.edit.news.insert(key.clone(), value);

        key
    }
}

impl<K: Clone, V: Clone, C: PushFront<K, V>, EC: Insert<K, V>> PushBack<K, V> for Recorder<C, EC> {
    fn push_back(&mut self, value: V) -> K {
        let key = self.container.push_front(value.clone());
        self.edit.news.insert(key.clone(), value);

        key
    }
}

/*pub trait PushFront<K, V> {
    fn push_front(&mut self, value: V) -> K;
}

pub trait PushBack<K, V> {
    fn push_back(&mut self, value: V) -> K;
}

pub trait PopFront<V> {
    fn pop_front(&mut self) -> Option<V>;
}

pub trait PopBack<V> {
    fn pop_back(&mut self) -> Option<V>;
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}*/
