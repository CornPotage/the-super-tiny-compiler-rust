use crate::ast::{ASTNode, ASTNodeType, NumberLiteral, StringLiteral};
use std::collections::HashMap;
use std::rc::Rc;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Visitor {
    fn enter(&self, node: &ASTNode, parent: Rc<ASTNode>) { println!("This is default implementation of enter.") }
    fn exit(&self, node: &ASTNode, parent: Rc<ASTNode>) { println!("This is default implementation of exit.") }
}

pub struct ProgramVisitFn;
pub struct NumberLiteralVisitFn;
pub struct StringLiteralVisitFn;
pub struct CallExpressionVisitFn;

impl Visitor for ProgramVisitFn {
    fn enter(&self, node: &ASTNode, parent: Rc<ASTNode>) {
        println!("enter program")
    }
}

impl Visitor for NumberLiteralVisitFn {
    fn enter(&self, node: &ASTNode, parent: Rc<ASTNode>) {
        println!("enter number")
    }
}

impl Visitor for StringLiteralVisitFn {
    fn enter(&self, node: &ASTNode, parent: Rc<ASTNode>) {
        println!("enter string")
    }
}

impl Visitor for CallExpressionVisitFn {
    fn enter(&self, node: &ASTNode, parent: Rc<ASTNode>) {
        println!("enter call-expression")
    }
}
