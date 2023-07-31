use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::hash::Hash;

pub struct Diff<'a, T> {
    added: Vec<&'a T>,
    removed: Vec<&'a T>,
    unchanged: Vec<&'a T>,
}

pub fn diff<'a, T>(old_vec: &'a [T], new_vec: &'a [T]) -> Diff<'a, T>
where
    T: Eq + Hash,
{
    let old_set: HashSet<&T, RandomState> = HashSet::from_iter(old_vec.iter());
    let new_set: HashSet<&T, RandomState> = HashSet::from_iter(new_vec.iter());

    let mut added: Vec<&T> = vec![];
    let mut removed: Vec<&T> = vec![];
    let mut unchanged: Vec<&T> = vec![];

    for item in new_set.iter() {
        if !old_set.contains(item) {
            added.push(item);
        }
    }

    for item in old_set.iter() {
        if !new_set.contains(item) {
            removed.push(item);
        } else {
            unchanged.push(item);
        }
    }

    Diff {
        added,
        removed,
        unchanged,
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::diff::diff;

    #[test]
    fn new_items_are_correct() {
        let vec1 = &vec![0, 1];
        let vec2 = &vec![0, 1, 2, 3];

        let difference = diff(vec1, vec2);

        let expected = vec![&2, &3];
        assert!(contain_all(&difference.added, &expected));
    }

    #[test]
    fn removed_items_are_correct() {
        let vec1 = &vec![0, 1, 2, 3];
        let vec2 = &vec![0, 1];

        let difference = diff(vec1, vec2);

        let expected = vec![&2, &3];
        assert!(contain_all(&difference.removed, &expected))
    }

    #[test]
    fn unchanged_items_are_correct() {
        let vec1 = &vec![0, 1, 2, 3];
        let vec2 = &vec![2, 3, 4, 5];

        let difference = diff(vec1, vec2);

        let expected = vec![&2, &3];
        assert!(contain_all(&difference.unchanged, &expected))
    }

    fn contain_all<T>(vec1: &Vec<T>, vec2: &Vec<T>) -> bool
    where
        T: Eq,
    {
        vec2.iter().all(|item| vec1.contains(item))
    }
}
