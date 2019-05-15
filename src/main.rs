pub mod department;
pub mod lexer;
pub mod user;

fn main() {
    let expression = lexer::tokenize("Add Panda Rust");
    println!("{:?}", expression)
}
