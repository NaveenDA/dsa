use std::collections::HashMap;
pub fn is_anagram(s: String, t: String) -> bool {
    let mut s_hash = HashMap::new();
    let mut t_hash = HashMap::new();
    // loop through each character in s and t
    for str in s.chars() {
        let contains_value = s_hash.get(&str);
        match contains_value {
            Some(val) => s_hash.insert(str, val + 1),
            None => s_hash.insert(str, 1),
        };
    }
    for str in t.chars() {
        let contains_value = t_hash.get(&str);
        match contains_value {
            Some(val) => t_hash.insert(str, val + 1),
            None => t_hash.insert(str, 1),
        };
    }
    // check if the two hashmaps are equal
    s_hash == t_hash
}
