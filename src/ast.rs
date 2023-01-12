use std::rc::Rc;
use strum_macros::Display;
use strum_macros::EnumIter;
use strum_macros::EnumString;
use strum_macros::IntoStaticStr;

#[derive(Eq, Hash, Debug, PartialEq, Clone, Copy, EnumString, Display, IntoStaticStr, EnumIter)]
pub enum  ASTNodeType {
    Root,
    Program,
    CallExpression,
    NumberLiteral,
    StringLiteral,
    ExpressionStatement,
    Identifier,
}

#[derive(Debug, PartialEq)]
pub struct NumberLiteral {
    pub(crate) node_type: ASTNodeType,
    pub(crate) value: String,
}

#[derive(Debug, PartialEq)]
pub struct StringLiteral {
    pub(crate) node_type: ASTNodeType,
    pub(crate) value: String,
}

#[derive(Debug, PartialEq)]
pub struct CallExpression {
    pub(crate) node_type: ASTNodeType,
    pub(crate) value: String,
    pub(crate) params: Vec<Rc<ASTNode>>,
}

#[derive(Debug, PartialEq)]
pub struct Root {
    pub(crate) node_type: ASTNodeType,
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub(crate) node_type: ASTNodeType,
    pub(crate) body: Vec<Rc<ASTNode>>,
}

#[derive(Debug, PartialEq)]
pub enum ASTNode {
    NumberLiteral(NumberLiteral),
    StringLiteral(StringLiteral),
    CallExpression(CallExpression),
    Program(Program),
    Root(Root),
}

impl ASTNode {
    pub fn get_node_type(&self) -> ASTNodeType {
        match self {
            ASTNode::Program(n) => n.node_type,
            ASTNode::NumberLiteral(p) => p.node_type,
            ASTNode::StringLiteral(p) => p.node_type,
            ASTNode::CallExpression(p) => p.node_type,
            ASTNode::Root(p) => p.node_type,
        }
    }
}

pub enum NewASTNode {
    NewProgram(NewProgram),
    CallExpressionWithCallee(CallExpressionWithCallee),
    ExpressionStatement(ExpressionStatement),
    Identifier(Identifier),
    NumberLiteral(NumberLiteral),
    StringLiteral(StringLiteral),
}

pub struct NewProgram {
    pub(crate) node_type: ASTNodeType,
    pub(crate) body: Vec<NewASTNode>,
}

pub struct ExpressionStatement {
    pub(crate) node_type: ASTNodeType,
    pub(crate) expression: Box<NewASTNode>,
}

pub struct CallExpressionWithCallee {
    pub(crate) node_type: ASTNodeType,
    pub(crate) callee: Identifier,
    pub(crate) arguments: Vec<NewASTNode>,
}

pub struct Identifier {
    pub(crate) node_type: ASTNodeType,
    pub(crate) name: String,
}
