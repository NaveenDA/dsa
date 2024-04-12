use std::collections::HashSet;

pub fn find_duplicate(items: &Vec<i32>)->bool{
    let mut hash = HashSet::new();
    for i in items{
        if hash.contains(i){
            return false;
        }
        hash.insert(i);
    }
    true
}