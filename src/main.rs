pub mod department;
pub mod lexer;
pub mod user;

fn main() {
    let expression = lexer::tokenize("Ad Panda Rust");
    println!("{:?}", expression)
}
