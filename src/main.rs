mod stack;
fn main() {
    println!("Hello, world!");
    let items = vec![1, 2, 3, 4, 5, 1];
    let is_dup = stack::find_duplicate(&items);
    println!("is contains dup {}", is_dup);
    println!(
        "is anagram {}",
        stack::is_anagram(String::from("god"), String::from("dog"))
    );

    println!(
        "is anagram {}",
        stack::is_anagram(String::from("cat"), String::from("ate"))
    );
}
