use crate::ast::{ASTNodeType, NewProgram, Program};
use crate::traverser::{TransformError, traverser};

pub fn transformer(program: Program) -> anyhow::Result<(), ()>{
    let mut new_ast =  NewProgram{
        node_type: ASTNodeType::Program,
        body: vec![]
    };

    let traverse_result = traverser(
        program,
    );

    match traverse_result {
        Ok(_) => return Ok(()),
        _ => Err(())
    }
}

#[cfg(test)]
mod transformer_tests {
    use std::rc::Rc;
    use crate::ast::{ASTNode, ASTNodeType, NumberLiteral, Program};
    use crate::transformer::transformer;

    #[test]
    fn test_transform_program() {
        use crate::traverser::mock_array_traverser;

        let ctx = mock_array_traverser::traverse_array_context();
        ctx.expect()
            .returning(|_, _, _| Ok(()));

        let param_ast_node1 = NumberLiteral { node_type: ASTNodeType::NumberLiteral, value: "number_literal1".to_string() };
        let param_ast_node_rc1 = Rc::new(ASTNode::NumberLiteral(param_ast_node1));

        let param_ast_node2 = NumberLiteral { node_type: ASTNodeType::NumberLiteral, value: "number_literal2".to_string() };
        let param_ast_node_rc2 = Rc::new(ASTNode::NumberLiteral(param_ast_node2));

        let program = Program {
            node_type: ASTNodeType::Program,
            body: vec![
                param_ast_node_rc1,
                param_ast_node_rc2,
            ],
        };
        // let program_rc = Rc::new(ASTNode::Program(program));

        let transform_result = transformer(program);
    }
}