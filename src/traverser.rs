use std::collections::HashMap;
use std::rc::Rc;
use crate::ast::{ASTNode, ASTNodeType, Program, Root};
use crate::visitor::{CallExpressionVisitFn, NumberLiteralVisitFn, ProgramVisitFn, StringLiteralVisitFn, Visitor};
use thiserror::Error;
use mockall_double::double;
#[cfg(test)]
use mockall::automock;

pub type Visitors = HashMap<ASTNodeType, Box<dyn Visitor>>;

pub fn traverser(program: Program) -> anyhow::Result<(), TransformError> {
    let mut visitor: Visitors = Visitors::new();

    visitor.insert(ASTNodeType::Program, Box::new(ProgramVisitFn));
    visitor.insert(ASTNodeType::NumberLiteral, Box::new(NumberLiteralVisitFn));
    visitor.insert(ASTNodeType::StringLiteral, Box::new(StringLiteralVisitFn));
    visitor.insert(ASTNodeType::CallExpression, Box::new(CallExpressionVisitFn));

    let root = Rc::new(ASTNode::Root(Root { node_type: ASTNodeType::Root }));
    let program_rc = Rc::new(ASTNode::Program(program));
    let traverse_result = travers_node(&visitor, program_rc, root);

    return traverse_result;
}

pub fn travers_node(visitors: &Visitors, node: Rc<ASTNode>, parent: Rc<ASTNode>) -> anyhow::Result<(), TransformError> {
    #[double]
    use array_traverser as inner;

    let visitor = visitors.get(&node.get_node_type());

    if let Some(methods) = visitor {
        methods.enter(&node, parent.clone())
    }

    let node_ref = node.as_ref();
    let traverse_child_result = match node_ref {
        ASTNode::Program(program) => {
            let body = (*program).body.clone();
            inner::traverse_array(visitors, body, node.clone())
        }
        ASTNode::CallExpression(call_expression) => {
            let params = (*call_expression).params.clone();
            inner::traverse_array(visitors, params, node.clone())
        }
        ASTNode::NumberLiteral(_) => Ok(()),
        ASTNode::StringLiteral(_) => Ok(()),
        _unknown_node => Err(TransformError::NoTransformTargetNode()).into()
    };

    if let Err(_) = traverse_child_result {
        return traverse_child_result;
    }

    if let Some(methods) = visitor {
        methods.exit(&node, parent.clone())
    }

    Ok(())
}

#[cfg_attr(test, automock)]
pub(super) mod array_traverser {
    use std::rc::Rc;
    use crate::ast::ASTNode;
    use super::{TransformError, travers_node, Visitors};

    pub fn traverse_array(visitors: &Visitors, array: Vec<Rc<ASTNode>>, parent: Rc<ASTNode>) -> anyhow::Result<(), TransformError> {
        for node in array {
            let traverse_result = travers_node(visitors, node, parent.clone());
            if let Err(_) = traverse_result {
                return traverse_result;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum TransformError {
    #[error("Error: The node is not transform target")]
    NoTransformTargetNode()
}

#[cfg(test)]
mod traverser_tests {
    use std::rc::Rc;
    use crate::ast::{ASTNode, ASTNodeType, CallExpression, NumberLiteral, Root, StringLiteral, Program};

    use crate::traverser::{TransformError, travers_node, Visitors};
    use crate::visitor::NumberLiteralVisitFn;
    use crate::visitor::{MockVisitor};

    #[test]
    fn test_travers_number_literal_node() {
        let mut visitor: Visitors = Visitors::new();

        let mut mock: MockVisitor = MockVisitor::new();
        mock.expect_enter()
            .times(1)
            .return_const(());
        mock.expect_exit()
            .times(1)
            .return_const(());
        visitor.insert(ASTNodeType::NumberLiteral, Box::new(mock));

        let root = Rc::new(ASTNode::Root(Root { node_type: ASTNodeType::Root }));
        let ast_node = NumberLiteral { node_type: ASTNodeType::NumberLiteral, value: "test".to_string() };
        let ast_node_rc = Rc::new(ASTNode::NumberLiteral(ast_node));

        let traverse_node_resul = travers_node(&visitor, ast_node_rc, root);
        let result_node = traverse_node_resul.unwrap();

        assert_eq!(result_node, ());
    }

    #[test]
    fn test_travers_string_literal_node() {
        let mut visitor: Visitors = Visitors::new();

        let mut mock: MockVisitor = MockVisitor::new();
        mock.expect_enter()
            .times(1)
            .return_const(());
        mock.expect_exit()
            .times(1)
            .return_const(());
        visitor.insert(ASTNodeType::StringLiteral, Box::new(mock));

        let root = Rc::new(ASTNode::Root(Root { node_type: ASTNodeType::Root }));
        let ast_node = StringLiteral { node_type: ASTNodeType::StringLiteral, value: "test".to_string() };
        let ast_node_rc = Rc::new(ASTNode::StringLiteral(ast_node));

        let traverse_node_resul = travers_node(&visitor, ast_node_rc, root);
        let result_node = traverse_node_resul.unwrap();

        assert_eq!(result_node, ());
    }

    #[test]
    fn test_travers_call_expression_node() {
        use crate::traverser::mock_array_traverser;

        let mut visitor: Visitors = Visitors::new();

        let mut mock: MockVisitor = MockVisitor::new();
        mock.expect_enter()
            .times(1)
            .return_const(());
        mock.expect_exit()
            .times(1)
            .return_const(());
        visitor.insert(ASTNodeType::CallExpression, Box::new(mock));

        let ctx = mock_array_traverser::traverse_array_context();
        ctx.expect()
            .times(1)
            .withf(|visitors: &Visitors, array: &Vec<Rc<ASTNode>>, parent: &Rc<ASTNode>| {
                {
                    let parent_node_type = parent.get_node_type();
                    if parent_node_type != ASTNodeType::CallExpression {
                        return false;
                    }

                    let node_ref = parent.as_ref();
                    let is_intended_type = match node_ref {
                        ASTNode::CallExpression(exp) => {
                            if "call_expression_test_value" == exp.value {
                                true
                            } else {
                                false
                            }
                        }
                        _ => false
                    };
                    if is_intended_type == false {
                        return false;
                    }
                }

                {
                    if 2 != array.len() {
                        return false;
                    }

                    let first_param = array[0].clone();
                    let first_param_node_type = first_param.get_node_type();
                    if first_param_node_type != ASTNodeType::NumberLiteral {
                        return false;
                    }

                    let node_ref = first_param.as_ref();
                    let is_intended_type = match node_ref {
                        ASTNode::NumberLiteral(exp) => {
                            if "number_literal1" == exp.value {
                                true
                            } else {
                                false
                            }
                        }
                        _ => false
                    };

                    if is_intended_type == false {
                        return false;
                    }
                }

                true
            })
            .returning(|_, _, _| Ok(()));

        let root = Rc::new(ASTNode::Root(Root { node_type: ASTNodeType::Root }));

        let param_ast_node1 = NumberLiteral { node_type: ASTNodeType::NumberLiteral, value: "number_literal1".to_string() };
        let param_ast_node_rc1 = Rc::new(ASTNode::NumberLiteral(param_ast_node1));

        let param_ast_node2 = NumberLiteral { node_type: ASTNodeType::NumberLiteral, value: "number_literal2".to_string() };
        let param_ast_node_rc2 = Rc::new(ASTNode::NumberLiteral(param_ast_node2));

        let ast_node = CallExpression {
            node_type: ASTNodeType::CallExpression,
            value: "call_expression_test_value".to_string(),
            params: vec![
                param_ast_node_rc1,
                param_ast_node_rc2,
            ],
        };
        let ast_node_rc = Rc::new(ASTNode::CallExpression(ast_node));

        let traverse_node_resul = travers_node(&visitor, ast_node_rc, root);
        let result_node = traverse_node_resul.unwrap();

        assert_eq!(result_node, ());
    }

    #[test]
    fn test_travers_program_node() {
        use crate::traverser::mock_array_traverser;

        let mut visitor: Visitors = Visitors::new();

        let mut mock: MockVisitor = MockVisitor::new();
        mock.expect_enter()
            .times(1)
            .return_const(());
        mock.expect_exit()
            .times(1)
            .return_const(());
        visitor.insert(ASTNodeType::Program, Box::new(mock));

        let ctx = mock_array_traverser::traverse_array_context();
        ctx.expect()
            .times(1)
            .withf(|visitors: &Visitors, array: &Vec<Rc<ASTNode>>, parent: &Rc<ASTNode>| {
                {
                    let parent_node_type = parent.get_node_type();
                    if parent_node_type != ASTNodeType::Program {
                        return false;
                    }

                    let node_ref = parent.as_ref();
                    let is_intended_type = match node_ref {
                        ASTNode::Program(exp) => true,
                        _ => false
                    };
                    if is_intended_type == false {
                        return false;
                    }
                }

                {
                    if 2 != array.len() {
                        return false;
                    }

                    let first_param = array[0].clone();
                    let first_param_node_type = first_param.get_node_type();
                    if first_param_node_type != ASTNodeType::NumberLiteral {
                        return false;
                    }

                    let node_ref = first_param.as_ref();
                    let is_intended_type = match node_ref {
                        ASTNode::NumberLiteral(exp) => {
                            if "number_literal1" == exp.value {
                                true
                            } else {
                                false
                            }
                        }
                        _ => false
                    };

                    if is_intended_type == false {
                        return false;
                    }
                }

                true
            })
            .returning(|_, _, _| Ok(()));

        let root = Rc::new(ASTNode::Root(Root { node_type: ASTNodeType::Root }));

        let param_ast_node1 = NumberLiteral { node_type: ASTNodeType::NumberLiteral, value: "number_literal1".to_string() };
        let param_ast_node_rc1 = Rc::new(ASTNode::NumberLiteral(param_ast_node1));

        let param_ast_node2 = NumberLiteral { node_type: ASTNodeType::NumberLiteral, value: "number_literal2".to_string() };
        let param_ast_node_rc2 = Rc::new(ASTNode::NumberLiteral(param_ast_node2));

        let ast_node = Program {
            node_type: ASTNodeType::Program,
            body: vec![
                param_ast_node_rc1,
                param_ast_node_rc2,
            ],
        };
        let ast_node_rc = Rc::new(ASTNode::Program(ast_node));

        let traverse_node_resul = travers_node(&visitor, ast_node_rc, root);
        let result_node = traverse_node_resul.unwrap();

        assert_eq!(result_node, ());
    }
}

