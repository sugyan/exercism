use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut char_counts = HashMap::new();
    word.to_lowercase()
        .chars()
        .for_each(|c| *char_counts.entry(c).or_insert(0) += 1);
    possible_anagrams
        .iter()
        .filter(|&w| {
            let mut counts = HashMap::new();
            w.to_lowercase()
                .chars()
                .for_each(|c| *counts.entry(c).or_insert(0) += 1);
            counts == char_counts && w.to_lowercase() != word.to_lowercase()
        })
        .map(|&s| s)
        .collect()
}
