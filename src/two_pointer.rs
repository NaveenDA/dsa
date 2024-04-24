use std::cmp::Ordering::{Equal, Greater, Less};

/**
* A function that checks if a string is a palindrome
*/
pub fn is_palindrome(s: String) -> bool {
    let str: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        let start = str[i];
        let end = str[j];
        if !start.is_alphanumeric() {
            i += 1;
            continue;
        }
        if !end.is_alphanumeric() {
            j -= 1;
            continue;
        }

        if str[i].to_ascii_lowercase() != str[j].to_ascii_lowercase() {
            return false;
        }
        j -= 1;
        i += 1;
    }
    true
}

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut start = 0;
    let mut end = numbers.len() - 1;
    while start < end {
        let curr = numbers[start] + numbers[end];
        match curr.cmp(&target) {
            Equal => return vec![(start + 1) as i32, (end + 1) as i32],
            Greater => end -= 1,
            Less => start += 1,
        }
    }
    return vec![-1, -1];
}
