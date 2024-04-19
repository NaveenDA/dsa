mod arr;
mod stack;
fn main() {
    let output = stack::valid_parentheses(String::from("]"));
    println!("{}", output);
    //  Your MinStack object will be instantiated and called as such:
    let mut obj: stack::MinStack = stack::MinStack::new();
    obj.push(-2);
    obj.push(0);
    obj.push(-3);
    println!("{}", obj.get_min());
    obj.pop();
    println!("{}", obj.top());
    println!("{}", obj.get_min());
}
