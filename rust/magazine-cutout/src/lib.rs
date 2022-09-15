// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_map : HashMap<&str,u32> = HashMap::new();
    let mut found_word_count = 0;

    for word in magazine {
        if let Some(count) = magazine_map.get_mut(word) {
            *count += 1;
        } else {
            magazine_map.insert(word, 1);
        }
    }
    
    for &word in note {
        if let Some(count) = magazine_map.get_mut(word) {
            if(*count > 0) {
                *count -= 1;
                found_word_count += 1;
            }
        }
    }

    note.len() == found_word_count

}
