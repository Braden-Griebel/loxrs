use crate::ast::{Expr, Stmt, Visitor};

struct AstPrinter {

}

impl Visitor<String> for AstPrinter {
    fn visit_expr(&mut self, expr: &mut Expr) -> String {
        match expr {
            Expr::Assign { .. } => { format!("")}
            Expr::Binary { left,
                operator,
                right } => {
                self.parenthesize(operator.lexeme.clone(), vec![left, right])
            }
            Expr::Call { .. } => {format!("")}
            Expr::Get { .. } => {format!("")}
            Expr::Grouping { expression } => {
                self.parenthesize(String::from("group"), vec![expression])
            }
            Expr::Literal { value } => { format!("{value}")}
            Expr::Logical { .. } => {format!("")}
            Expr::Set { .. } => {format!("")}
            Expr::Super { .. } => {format!("")}
            Expr::This { .. } => {format!("")}
            Expr::Unary { operator, right } => {
                self.parenthesize(operator.lexeme.clone(), vec![right])
            }
            Expr::Variable { .. } => {format!("")}
        }
    }

    fn visit_stmt(&mut self, stmt: &mut Stmt) -> String {
        match stmt{
            Stmt::Block { .. } => {format!("")}
            Stmt::Class { .. } => {format!("")}
            Stmt::Expression { .. } => {format!("")}
            Stmt::Function { .. } => {format!("")}
            Stmt::If { .. } => {format!("")}
            Stmt::Print { .. } => {format!("")}
            Stmt::Return { .. } => {format!("")}
            Stmt::Variable { .. } => {format!("")}
            Stmt::While { .. } => {format!("")}
        }
    }
}

impl AstPrinter {
    fn print(&mut self, expr: &mut Expr)->String{
        self.visit_expr(expr)
    }
    fn parenthesize(&mut self, name:String, exprs: Vec<&mut Expr>)->String{
        let mut paren_str = String::new();
        paren_str.push_str("(");
        for expr in exprs {
            paren_str.push_str(" ");
            paren_str.push_str(&self.visit_expr(expr))
        }
        paren_str.push_str(")");
        paren_str
    }
}