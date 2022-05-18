use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
pub fn is_prefix<T: PartialEq>(list1: &[T], list2: &[T]) -> bool {
    if list1.len() > list2.len() {
        return false;
    }
    list1.iter().zip(list2.iter()).all(|(x, y)| x == y)
}

pub fn is_sublist<T: PartialEq>(list1: &[T], list2: &[T]) -> bool {
    (0..list2.len() + 1).any(|index| is_prefix(list1, &list2[index..]))
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let s1 = is_sublist(_first_list, _second_list);
    let s2 = is_sublist(_second_list, _first_list);
    match (s1, s2) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}
