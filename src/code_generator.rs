use crate::ast::{NewASTNode, NewProgram};

pub fn generate_code(new_node: NewASTNode) -> String {
    generate(&new_node)
}

fn generate(new_node: &NewASTNode) -> String {
    match new_node {
        NewASTNode::NewProgram(new_program) => {
            let body = &new_program.body;

            let codes = body.iter().map(generate).collect::<Vec<String>>();

            codes.join("\n")
        }
        NewASTNode::ExpressionStatement(expression_statement) => {
            let expression = &expression_statement.expression;
            let code = generate(expression);

            code
        }
        NewASTNode::CallExpressionWithCallee(call_expression_with_callee) => {
            let callee = &call_expression_with_callee.callee;
            let arguments = &call_expression_with_callee.arguments;

            let codes = arguments.iter().map(generate).collect::<Vec<String>>();

            format!("{}({})", callee.name, codes.join(","))
        }
        NewASTNode::Identifier(identifier) => format!("'{}'", identifier.name),
        NewASTNode::NumberLiteral(number_literal) => format!("'{}'", number_literal.value),
        NewASTNode::StringLiteral(string_literal) => format!("'{}'", string_literal.value),
    }
}
