use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct KeyValueDiff<'a, K, V> {
    removed: Vec<&'a (K, V)>,
    added: Vec<&'a (K, V)>,
    changed: Vec<&'a (K, V)>,
    unchanged: Vec<&'a (K, V)>,
}

pub fn diff_key_value<'a, K, V>(
    iter_old: impl Iterator<Item = &'a (K, V)>,
    iter_new: impl Iterator<Item = &'a (K, V)>,
) -> KeyValueDiff<'a, K, V>
    where
        K: Hash + Eq,
        V: Eq,
{
    todo!()
    // let old_keys: Vec<&K> = iter_old.iter().map(|i| i.0).collect();
    // let new_keys: Vec<&K> = iter_new.iter().map(|i| i.0).collect();

    // let old_keys_set: HashSet<&K, RandomState> = HashSet::from_iter(old_keys);
    // let new_keys_set: HashSet<&K, RandomState> = HashSet::from_iter(new_keys);

    // let mut added = vec![];
    // let mut removed = vec![];

    // for entry in iter_new {
    //     if old_keys_set.contains(&entry.0) {
    //         added.push(entry);
    //     }
    // }

    // KeyValueDiff {
    //     added,
    //     removed,
    //     changed: vec![],
    //     unchanged: vec![],
    // }

    // let old_set: HashSet<&T, RandomState> = HashSet::from_iter(old_vec.iter());
    // let new_set: HashSet<&T, RandomState> = HashSet::from_iter(new_vec.iter());
    //
    // let mut added: Vec<&T> = vec![];
    // let mut removed: Vec<&T> = vec![];
    //
    // for (key) in new_set.iter() {
    //     if !old_set.contains(key) {
    //         added.push(key);
    //     }
    // }
    //
    // for item in old_set.iter() {
    //     if !new_set.contains(item) {
    //         removed.push(item);
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use crate::utils::diff_key_value::diff_key_value;

    #[test]
    fn new_entries_are_correctly_found() {
        let entries_a: [(&str, u32); 2] = [
            ("a", 10),
            ("b", 10),
        ];

        let entries_b: [(&str, u32); 4] = [
            ("a", 10),
            ("b", 20),
            ("c", 30),
            ("d", 40),
        ];

        let diff = diff_key_value::<&str, u32>(entries_a.iter(), entries_b.iter());
        assert_eq!(diff.added, vec![
            &("c", 30_u32),
            &("d", 40_u32),
        ]);
    }
}
