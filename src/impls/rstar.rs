use rstar::{RTree, RTreeObject};

use crate::map::{Get, Insert, IntoIter, Keyed, Map, Remove};

impl<K: RTreeObject> Map for RTree<K> {
    type Item = ();
}

impl<K: RTreeObject> Keyed for RTree<K> {
    type Key = K;
}

impl<K: RTreeObject + PartialEq> Get<K> for RTree<K> {
    #[inline(always)]
    fn get(&self, key: &K) -> Option<&()> {
        RTree::contains(self, key).then_some(&())
    }
}

impl<K: RTreeObject> Insert<K> for RTree<K> {
    #[inline(always)]
    fn insert(&mut self, key: K, _value: ()) {
        RTree::insert(self, key);
    }
}

impl<K: RTreeObject + PartialEq> Remove<K> for RTree<K> {
    #[inline(always)]
    fn remove(&mut self, key: &K) -> Option<()> {
        RTree::remove(self, key).map(|_| ())
    }
}

pub struct MapIntoIter<K: RTreeObject>(rstar::iterators::IntoIter<K>);

impl<K: RTreeObject> Iterator for MapIntoIter<K> {
    type Item = (K, ());

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|k| (k, ()))
    }
}

impl<K: RTreeObject> IntoIter<K> for RTree<K> {
    type IntoIter = MapIntoIter<K>;

    fn into_iter(self) -> MapIntoIter<K> {
        MapIntoIter(IntoIterator::into_iter(self))
    }
}

#[cfg(test)]
mod tests {
    use rstar::RTree;

    use crate::Recorder;

    impl crate::recorder::tests::FromUsize for (i32, i32) {
        fn from_usize(u: usize) -> (i32, i32) {
            (u as i32, 0)
        }
    }

    #[test]
    fn test_apply_edit_on_set() {
        let recorder =
            Recorder::<(i32, i32), (), RTree<(i32, i32)>, RTree<(i32, i32)>>::new(RTree::new());
        crate::recorder::tests::test_apply_edit_on_set(recorder);
    }

    #[test]
    fn test_insert_and_remove_on_set() {
        let recorder =
            Recorder::<(i32, i32), (), RTree<(i32, i32)>, RTree<(i32, i32)>>::new(RTree::new());
        crate::recorder::tests::test_insert_and_remove_on_set(recorder);
    }

    #[test]
    fn test_edit_undo_redo_on_set() {
        crate::undoredo::tests::test_edit_undo_redo_on_set::<
            (i32, i32),
            RTree<(i32, i32)>,
            RTree<(i32, i32)>,
        >(RTree::new());
    }
}
