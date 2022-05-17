use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut set: HashSet<&str> = HashSet::new();
    for string in possible_anagrams.iter() {
        let current_word = word.to_lowercase();
        let current_anagram = string.to_lowercase();
        if current_anagram == current_word {
            continue;
        }
        let mut word_chars = current_word.chars().collect::<Vec<char>>();
        let mut failed = false;
        for c in current_anagram.chars() {
            if word_chars.contains(&c) {
                if let Some(index) = word_chars.iter().position(|w| w == &c) {
                    word_chars.remove(index);
                    continue;
                }
            }
            failed = true;
            break;
        }
        if word_chars.len() == 0 && !failed {
            set.insert(string);
        }
    }

    return set;
}
