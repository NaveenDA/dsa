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

/**
 * A function that will return index of the array where target is by adding two values
 * of those incides
 */
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_map: HashMap<i32, i32> = HashMap::new();
    let num_ref = &nums;

    for (i, el) in num_ref.into_iter().enumerate() {
        let current_tagert: i32 = target - el;
        let got_res = num_map.get(&current_tagert);
        match got_res {
            Some(val) => return vec![i as i32, *val],
            None => num_map.insert(*el, i as i32),
        };
    }
    return vec![];
}

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let hash_az: Vec<usize> = vec![0; 26];
    let mut res: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
    for str in strs {
        let char_vec: Vec<u8> = str.chars().map(|c| c as u8).collect();
        let mut hash_az_copy = hash_az.clone();
        for c in char_vec {
            let acsii_a: u8 = b'a';
            let diff = c as usize - acsii_a as usize;
            hash_az_copy[diff] = hash_az_copy[diff] + 1
        }
        let hash_key: Vec<u8> = hash_az_copy.iter().map(|x| *x as u8).collect();
        match res.get_mut(&hash_key) {
            Some(v) => v.push(str),
            None => {
                res.insert(hash_key, vec![str]);
            }
        }
    }
    res.into_iter().map(|(_, value)| value).collect()
}

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freq = HashMap::new();
    let mut bc_freq = vec![vec![]; nums.len() + 1];

    for num in nums {
        *freq.entry(num).or_insert(0) += 1;
    }

    for (n, c) in freq {
        bc_freq[c].push(n);
    }
    let mut result = vec![];
    for i in bc_freq.iter().rev() {
        for &n in i {
            result.push(n);
        }
        if result.len() == (k as usize) {
            return result;
        }
    }
    return result;
}

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let k = nums.len();
    let mut prefix = vec![1; k];
    let mut suffix = vec![1; k];
    let mut result = vec![1; k];
    let mut j = k - 1;
    for index in 1..k {
        prefix[index] = prefix[index - 1] * nums[index - 1];
        suffix[j - 1] = suffix[j] * nums[j];
        j -= 1;
    }
    for i in 0..k {
        result[i] = prefix[i] * suffix[i]
    }

    result
}

pub fn encode_string(inputs: Vec<String>) -> String {
    let mut output = String::new();
    for input in inputs {
        let count = input.chars().count();
        let new_string = format!("#{count}{input}").to_owned();
        output.push_str(&new_string);
    }
    output
}
// strs = #3eat#3tea#3tan#3ate#3nat#3bat
pub fn decode_string(strs: String) -> Vec<String> {
    let mut output: Vec<String> = vec![];
    let chars: Vec<char> = strs.chars().collect();
    let mut i = 0;
    let count = chars.len();
    while i < count {
        if chars[i] == '#' && chars[i + 1].is_numeric() {
            let mut j = i + 1;
            let mut length_str = String::new();
            while j < count && chars[j].is_numeric() {
                length_str.push(chars[j]);
                j += 1;
            }
            let end = length_str.parse().unwrap_or(0);
            let new_str: String = chars[j..(j + end)].into_iter().collect();
            output.push(new_str);
            i += end + 2;
        } else {
            i += 1;
        }
    }
    output
}
