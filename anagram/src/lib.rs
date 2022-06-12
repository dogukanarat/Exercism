use std::collections::HashSet;
use std::collections::HashMap;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result : HashSet<&'a str> = HashSet::new();
    let mut character_map : HashMap<u8, i32> = HashMap::new();
    let mut lowercase_word = word.to_lowercase().bytes().collect::<Vec<u8>>();
    let lowercase_word_unsorted = word.to_lowercase().bytes().collect::<Vec<u8>>();

    lowercase_word.sort();

    lowercase_word.iter().for_each(|character| {
        let count = character_map.entry(*character).or_insert(1);
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
            if let Ok(string) = String::from_utf8(lowercase_word_unsorted.clone())
            {
                if word.len() != string.chars().count() {
                    continue;
                }
                if !word.is_ascii()
                {
                    continue;
                }
                if string != lowercase_possible_anagram {
                    result.insert(possible_anagram);
                }
            }
        }

    }

    result
}
