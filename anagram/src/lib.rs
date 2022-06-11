use std::collections::HashSet;
use std::collections::HashMap;
use std::char::*;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result : HashSet<&'a str> = HashSet::new();
    let mut character_map : HashMap<u8, i32> = HashMap::new();
    let lowercase_word = word.to_lowercase();

    lowercase_word.bytes().for_each(|b| {
        let count = character_map.entry(b).or_insert(1);
        *count += 1;
    });

    for possible_anagram in possible_anagrams {
        let lowercase_possible_anagram = possible_anagram.to_lowercase();
        if lowercase_possible_anagram.len() != word.len() {
            continue;
        }

        let mut temp_character_map : HashMap<u8, i32> = HashMap::new();

        lowercase_possible_anagram.bytes().for_each(|b| {
            let count = temp_character_map.entry(b).or_insert(1);
            *count += 1;
        });

        if character_map == temp_character_map {
            if lowercase_word != lowercase_possible_anagram
            {
                result.insert(possible_anagram);
            }
            
        }

    }

    result
}
