use esbuild_rs::ast::Operator;

fn main() {
    println!("{}", Operator::Lowest < Operator::Lowest);
}
