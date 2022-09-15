use std::collections::HashSet;
use std::collections::HashMap;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result : HashSet<&'a str> = HashSet::new();
    let mut character_map : HashMap<char, i32> = HashMap::new();

    let lowercase_word = word.to_lowercase();

    lowercase_word.chars().for_each(|c| {
        let count = character_map.entry(c).or_insert(0);
        *count += 1;
    });

    for possible_anagram in possible_anagrams {
        let lowercase_possible_anagram = possible_anagram.to_lowercase();
        if lowercase_possible_anagram.len() != word.len() {
            continue;
        }

        let mut temp_character_map : HashMap<char, i32> = HashMap::new();

        lowercase_possible_anagram.chars().for_each(|b| {
            let count = temp_character_map.entry(b).or_insert(0);
            *count += 1;
        });

        if character_map == temp_character_map {
            if lowercase_word != lowercase_possible_anagram {
                result.insert(possible_anagram);
            }
        }

    }

    result
}
