use crate::department::department::Department;
use crate::user::User;
use self::regex::Regex;

extern crate regex;

#[derive(Debug)]
pub enum Operation {
    Add,
    Remove,
}

#[derive(Debug)]
pub struct Operand {
    value: String
}

#[derive(Debug)]
pub struct Expression {
    operation: Operation,
    user: Operand,
    department: Operand,
}

pub fn tokenize(string: String) -> Expression {
    let words = string.split(" ").collect::<Vec<_>>();

    if words.len() != 3 { panic!("Expression must be 3 words length!") }

    let operation = match words[0] {
        "Add" => Operation::Add,
        "Remove" => Operation::Remove,
        _ => panic!("Undefined operation!")
    };

//    let user = match words[1] {
//        "[A-Za-z]" => Operand { value: words[1].to_string() },
//        _ => panic!("Operand must contain only latin symbols!")
//    };

    let re = Regex::new("[A-Za-z]+").unwrap();
    assert!(re.is_match(words[1]));

    if Regex::new("[A-Za-z]+").unwrap().is_match(words[1]) {
        let user = words[1];
    }

    if Regex::new("[A-Za-z]+").unwrap().is_match(words[2]) {
        let department = words[1];
    }

    return Expression {
        operation,
        user,
        department
    };
}
