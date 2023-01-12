use crate::compiler::compiler;
// use crate::visitor::tra_test;

mod token;
mod tokenizer;
mod compiler;
mod parser;
mod ast;
mod traverser;
mod visitor;
mod transformer;
mod code_generator;

fn main() -> anyhow::Result<()> {
    let input= "(add 1 2)".to_string();
    compiler(input)
}
