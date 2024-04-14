use std::collections::{HashMap, HashSet};

/**
 * This method is used to check a vec contains duplicates or not
 */
pub fn find_duplicate(items: &Vec<i32>) -> bool {
    let mut hash = HashSet::new();
    for i in items {
        if hash.contains(i) {
            return false;
        }
        hash.insert(i);
    }
    true
}

/**
 * A function to check two string is anagram or not
 */
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

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_map:HashMap<i32, i32> = HashMap::new();
    let num_ref = &nums;
   
    for (i,el) in num_ref.into_iter().enumerate() {
        let current_tagert: i32 = target - el;
        let got_res = num_map.get(&current_tagert);
        match got_res {
            Some(val) => return vec![i as i32, *val],
            None => num_map.insert( *el, i as i32)
        };
    }
    return vec![];
}
