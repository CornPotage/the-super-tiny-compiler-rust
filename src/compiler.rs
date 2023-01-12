use crate::parser::parser;
use crate::tokenizer::tokenizer;
use crate::transformer::transformer;

pub fn compiler(code: String) -> anyhow::Result<()> {
    let tokens = tokenizer(code)?;
    let program = parser(tokens)?;
    let new_program = transformer(program);

    Ok(())
}