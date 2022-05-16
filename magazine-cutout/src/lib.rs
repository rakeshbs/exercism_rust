// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_hash = magazine.iter().fold(HashMap::new(), |mut mhash, word| {
        *mhash.entry(word).or_insert(0) += 1;
        mhash
    });

    note.iter().all(|word| match magazine_hash.get_mut(word) {
        Some(count) if *count > 0 => {
            *count -= 1;
            true
        }
        _ => false,
    })
}
