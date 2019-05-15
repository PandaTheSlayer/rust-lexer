pub mod lexer;
pub mod user;
pub mod department;

fn main() {
    let expression = lexer::tokenize("Add Panda Rust".to_string());
    println!("{:?}", expression)
}
