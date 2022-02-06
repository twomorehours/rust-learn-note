use sqlparser::{
    ast::{SetExpr, Statement},
    dialect::GenericDialect,
    parser::Parser,
};

fn main() {
    let mut ast = Parser::parse_sql(
        &GenericDialect {},
        "select a from abc where 1",
    )
    .unwrap();
    if ast.len() != 1 {
        panic!("expected")
    }

    if let Statement::Query(query) = ast.pop().unwrap() {
        if let SetExpr::Select(select) = query.body {
            println!("projection: {:?}", select.projection);
            let project = select.projection;


            println!("");
            println!("from: {:?}", select.from);
            println!("");
            println!("selection: {:?}", select.selection);

            return;
        }
    }

    panic!("expected");
}
