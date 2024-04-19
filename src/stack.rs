use std::{cmp, collections::HashMap, vec};

pub fn valid_parentheses(s: String) -> bool {
    let mut brackets: HashMap<char, char> = HashMap::new();
    brackets.insert(')', '(');
    brackets.insert('}', '{');
    brackets.insert(']', '[');
    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
        let has_end: Option<&char> = brackets.get(&c);
        match has_end {
            Some(&v) => {
                if (stack.len() > 0 && stack[stack.len() - 1] != v) || stack.len() <= 0 {
                    return false;
                }
                stack.pop();
            }
            None => {
                stack.push(c);
            }
        }
    }
    stack.len() == 0
}

pub struct MinStack {
    main: Vec<i32>,
    min: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    pub fn new() -> Self {
        Self {
            main: vec![],
            min: vec![],
        }
    }

    pub fn push(&mut self, val: i32) {
        let last = self.min.last();
        let min = match last {
            Some(&v) => v,
            None => val,
        };
        self.main.push(val);
        self.min.push(cmp::min(val, min));
    }

    pub fn pop(&mut self) -> () {
        self.min.pop();
        self.main.pop();
    }

    pub fn top(&self) -> i32 {
        match self.main.last() {
            Some(&v) => v,
            None => 0,
        }
    }

    pub fn get_min(&self) -> i32 {
        match self.min.last() {
            Some(&v) => v,
            None => 0,
        }
    }
}

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = vec![];
    for token in tokens {
        match token.as_str() {
            "+" | "-" | "*" | "/" => {
                let b = stack.pop().unwrap_or_default();
                let a = stack.pop().unwrap_or_default();
                let result = match token.as_str() {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => a / b,
                    _ => 0,
                };
                stack.push(result);
            }
            _ => stack.push(token.parse().unwrap_or_default()),
        }
    }
    stack.last().copied().unwrap_or_default()
}
