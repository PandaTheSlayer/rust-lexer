use failure::{bail, err_msg, format_err, Error, Fallible, ResultExt};
use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
pub enum Operation {
    Add,
    Remove,
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Fallible<Self> {
        Ok(match s {
            "Add" => Operation::Add,
            "Remove" => Operation::Remove,
            _ => bail!("panda"),
        })
    }
}

#[derive(Debug)]
pub struct Operand<'a>(&'a str);

impl<'a> Operand<'a> {
    fn new_with_regex(regex: &Regex, value: &'a str) -> Fallible<Self> {
        if regex.is_match(value) {
            Ok(Operand(value))
        } else {
            Err(format_err!(
                "operand regex validation fail, regex: {:#?}, operand: {}",
                regex,
                value
            ))
        }
    }
}

#[derive(Debug)]
pub struct Expression<'a> {
    operation: Operation,
    user: Operand<'a>,
    department: Operand<'a>,
}

pub fn tokenize<'a>(string: &'a str) -> Fallible<Expression<'a>> {
    let mut words = string.split_whitespace();
    let regex = Regex::new("[A-Za-z]+").context("regex build error")?;
    let operation = words
        .next()
        .map(Operation::from_str)
        .ok_or_else(|| err_msg("operation not found"))??;
    let user = words
        .next()
        .map(|user| Operand::new_with_regex(&regex, user))
        .ok_or_else(|| err_msg("user not found"))??;
    let department = words
        .next()
        .map(|department| Operand::new_with_regex(&regex, department))
        .ok_or_else(|| err_msg("department not found"))??;
    Ok(Expression {
        operation,
        user,
        department,
    })
}
