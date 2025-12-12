pub trait Collection {
    type Item;
}

pub trait Keyed {
    type Key;
}

pub trait Get<K>: Collection {
    fn get(&self, key: &K) -> Option<&Self::Item>;
}

pub trait Insert<K>: Collection {
    fn insert(&mut self, key: K, value: Self::Item);
}

pub trait Remove<K>: Collection {
    fn remove(&mut self, key: &K) -> Option<Self::Item>;
}

pub trait Push<K>: Collection {
    fn push(&mut self, value: Self::Item) -> K;
}

/*pub trait Iter<K>: Collection + Keyed {
    type Iter<'a>: Iterator<Item = (&'a K, &'a Self::Item)>
    where
        Self: 'a,
        K: 'a;

    fn iter(&self) -> Self::Iter<'_>;
}*/

pub trait IntoIter<K>: Collection + Keyed {
    type IntoIter: Iterator<Item = (K, Self::Item)>;

    fn into_iter(self) -> Self::IntoIter;
}
