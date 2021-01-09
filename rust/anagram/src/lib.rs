use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut char_counts = HashMap::new();
    let word_lower = word.to_lowercase();
    word_lower
        .chars()
        .for_each(|c| *char_counts.entry(c).or_insert(0) += 1);
    let is_anaglam = |w: &str| -> bool {
        if w.len() != word.len() {
            return false;
        }
        let w_lower = w.to_lowercase();
        if w_lower == word_lower {
            return false;
        }
        let mut counts = HashMap::new();
        w_lower
            .chars()
            .for_each(|c| *counts.entry(c).or_insert(0) += 1);
        counts == char_counts
    };
    possible_anagrams
        .iter()
        .filter(|&w| is_anaglam(&w))
        .cloned()
        .collect()
}
