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

    let result = stack::two_sum(vec![0, 7, 11, 15, 2], 9);
    println!("The result of two sum {:?}", result);

    let input: Vec<String> = vec![
        String::from("eat"),
        String::from("tea"),
        String::from("tan"),
        String::from("ate"),
        String::from("nat"),
        String::from("bat"),
    ];
    let result = stack::group_anagrams(input);
    println!("The result of the group anagrams is {:?}", result);

    let result = stack::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
    println!("The result of the top k frequent {:?}", result);

    let result = stack::top_k_frequent(vec![1], 1);
    println!("The result of the top k frequent {:?}", result);

    let result = stack::product_except_self(vec![1, 2, 3, 4]);
    println!("The result of the product except itself is {:?}", result);

    let input: Vec<String> = vec![
        String::from("eat"),
        String::from("tea"),
        String::from("12tan"),
        String::from("ate"),
        String::from("nat from the world"),
        String::from("hello!f#2"),
        String::from("#2"),
    ];
    let output = stack::encode_string(input);
    println!("The output of the encode string will be {:?}", output);
    let output = stack::decode_string(output);
    println!("The output of the decode string will be {:?}", output);

    let board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    println!("{}", stack::is_valid_sudoku(board));
}
