use std::{collections::HashMap, thread};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let joined = input.join("").to_lowercase();
    let thread_count = worker_count.min(joined.len());

    let slice_size = (joined.len() + worker_count - 1) / worker_count;

    let mut handles = Vec::new();

    for i in 0..thread_count {
        let start = slice_size * i;
        let mut end = start + slice_size;
        if end > joined.len() {
            end = joined.len();
        }
        let mut hash: HashMap<char, usize> = HashMap::new();
        let maybe_slice = joined.get(start..end);
        if maybe_slice.is_some() {
            let slice = maybe_slice.unwrap().to_string();
            let handle = thread::spawn(move || {
                for c in slice.chars() {
                    if c.is_alphabetic() {
                        *hash.entry(c).or_insert(0) += 1;
                    }
                }
                hash
            });
            handles.push(handle);
        }
    }

    let mut frequency: HashMap<char, usize> = HashMap::new();

    for handle in handles {
        let hashmap: HashMap<char, usize> = handle.join().unwrap();
        for (k, v) in hashmap.iter() {
            *frequency.entry(*k).or_insert(0) += v;
        }
    }
    frequency
}
